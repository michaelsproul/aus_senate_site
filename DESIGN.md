/
    choose a state page

GET /:state:/
    set up page for running an election for that state

    initially: just include a text box for typing people's names to exclude

POST /:state:/
    run election with POSTed data => display result page, or error

result caching? nah, too complicated initially
