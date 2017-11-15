/
    choose a state page

GET /:state:/
    set up page for running an election for that state

    initially: just include a text box for typing people's names to exclude

POST /:state:/
    run election with POSTed data => display result page, or error

result caching? nah, too complicated initially

GET /:state
    set up page
On form submit:
    * Change page contents to "Loading..."
    * Add job to run queue (only run 1 job at a time initially).
    * On callback, load the page returned by POST /:state

    Extensions:
    * Proof of work required to submit jobs.
    * Would be good to have a way to track job progress.
