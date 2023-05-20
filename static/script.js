function nextQuestion(answer) {
    // Hide the current question
    var currentQuestion = document.getElementById('question1');
    currentQuestion.style.display = 'none';

    // Show the next question based on the selected answer
    var nextQuestion;
    switch (answer) {
        case 'learn':
            nextQuestion = document.getElementById('question2');
            break;
        case 'project':
            nextQuestion = document.getElementById('question3');
            break;
        case 'explore':
            nextQuestion = document.getElementById('question4');
            break;
        case 'experience':
            nextQuestion = document.getElementById('question5');
            break;
        case 'experience':
            nextQuestion = document.getElementById('question5');
            break;
        default:
            // Handle any other cases or error conditions
            break;
    }

    // Show the next question
    if (nextQuestion) {
        nextQuestion.style.display = 'block';
    }
}