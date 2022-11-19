#!/bin/bash

USERS_ENDPOINT='127.0.0.1:7878/users'
LOGIN_ENDPOINT='127.0.0.1:7878/login'
QUESTIONS_ENDPOINT='127.0.0.1:7878/questions'

OK_STATUS="200"
CREATED_STATUS="201"
NO_CONTENT_STATUS="204"
EMPTY_BODY="[]"

EXIT_STATUS=0


echo "Creating common user..."
curl --fail --location --request POST $USERS_ENDPOINT \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "rob.pike.common@gmail.com",
    "password": "concurrency",
    "first_name": "Rob",
    "last_name": "Pike"
}'



echo "Listing all questions..."
list_questions_resp=$(curl --location --request GET $QUESTIONS_ENDPOINT)
if [ ${#list_questions_resp} != ${#EMPTY_BODY} ]
then
    echo "########################## ERROR ##########################"
    echo "Listing questions error. Should be an empty array"
    EXIT_STATUS=1
fi



echo "Obtaining token for common user..."
login_resp_body=$(curl --location --request POST $LOGIN_ENDPOINT \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "rob.pike.common@gmail.com",
    "password": "concurrency"
}')
echo "Login RESPONSE body: $login_resp_body"
capture='\([^\"]*\)'
token_string=$(echo $login_resp_body | sed "s/{.*\"token\":\"$capture.*}/\1/g")



echo "Creating a new question..."
create_question_resp=$(curl --location --request POST $QUESTIONS_ENDPOINT \
--header "Authorization: Token $token_string" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Title of a question/complaint/order",
    "content": "If created by common user rather than moderator, all the bad words and swearing, including '\''shit'\'' will be censored with * by a 3rd party service. NB! the BadWords service we are currecntly using cannot process parenthesis",
    "tags": ["this", "string", "array", "field", "is", "optional"],
    "status": "Pending"
}')
echo "questions CREATE response: $create_question_resp"
new_question_id=$(echo $create_question_resp | sed "s/{.*\"_id\":\"$capture.*}/\1/g")



echo "Getting question with id $new_question_id"
get_question_operation_resp_satus_code=$(curl -o /dev/null -w "%{http_code}" --location --request GET "$QUESTIONS_ENDPOINT/$new_question_id")
if [ $get_question_operation_resp_satus_code != $OK_STATUS ]
then
    echo "########################## ERROR ##########################"
    echo "Expected status_code $OK_STATUS. Actual status code: $get_question_operation_resp_satus_code"
    EXIT_STATUS=1
fi



echo "Listing all questions again..."
list_questions_resp=$(curl --location --request GET $QUESTIONS_ENDPOINT)
if [ ${#list_questions_resp} == ${#EMPTY_BODY} ]
then
    echo "########################## ERROR ##########################"
    echo "Listing questions error. The body is empty."
    EXIT_STATUS=1
fi



echo "Updating question with id $new_question_id"
update_question_status_code=$(curl -o /dev/null -w "%{http_code}" --location --request PUT "$QUESTIONS_ENDPOINT/$new_question_id" \
--header "Authorization: Token $token_string" \
--header 'Content-Type: application/json' \
--data-raw '{
    "title": "Title of a question/complaint/order [RESOLVED]",
    "content": "If updated by common user rather than moderator, all the bad words and swearing, including '\''shit'\'' will be censored with * by a 3rd party service. NB! the BadWords service we are currecntly using cannot process parenthesis",
    "tags": ["this", "string", "array", "field", "is", "optional"],
    "status": "Resolved"
}')
if [ $update_question_status_code != $NO_CONTENT_STATUS ]
then
    echo "########################## ERROR ##########################"
    echo "Update question operation returned unexpected status code: $update_question_status_code"
    EXIT_STATUS=1
fi



echo "Deleting question with id $new_question_id"
delete_question_status_code=$(curl -o /dev/null -w "%{http_code}" --location --request DELETE "$QUESTIONS_ENDPOINT/$new_question_id" \
--header "Authorization: Token $token_string")
if [ $delete_question_status_code != $NO_CONTENT_STATUS ]
then
    echo "########################## ERROR ##########################"
    echo "Delete question operation returned unexpected status code: $delete_question_status_code"
    EXIT_STATUS=1
fi



echo "Listing all questions yet again..."
list_questions_resp=$(curl --location --request GET $QUESTIONS_ENDPOINT)
if [ ${#list_questions_resp} != ${#EMPTY_BODY} ]
then
    echo "########################## ERROR ##########################"
    echo "Listing questions error. Should be an empty array after the test run."
    EXIT_STATUS=1
fi



echo "Creating a moderator user"
create_moderator_user_operation_status_code=$(curl -o /dev/null -w "%{http_code}" --location --request POST $USERS_ENDPOINT \
--header "Authorization: $MODERATOR_AUTH_KEY" \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "rob.pike.moderator@gmail.com",
    "password": "concurrency",
    "first_name": "Rob",
    "last_name": "Pike",
    "is_moderator": true
}')
if [ $create_moderator_user_operation_status_code != $CREATED_STATUS ]
then
    echo "########################## ERROR ##########################"
    echo "Create moderator user operation returned unexpected status code: $create_moderator_user_operation_status_code"
    EXIT_STATUS=1
fi


echo "Obtaining token for moderator.."
login_operation_status_code=$(curl -o /dev/null -w "%{http_code}" --location --request POST $LOGIN_ENDPOINT \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "rob.pike.moderator@gmail.com",
    "password": "concurrency"
}')
if [ $login_operation_status_code != $CREATED_STATUS ]
then
    echo "########################## ERROR ##########################"
    echo "Login moderator user operation returned unexpected status code: $login_operation_status_code"
    EXIT_STATUS=1   
fi



# RESULTS OF THE SELF-CLEANING RUN
if [ $EXIT_STATUS != 0 ]
then
    echo "FAILURE"
    exit 1
fi

echo "SUCCESS"
exit 0
