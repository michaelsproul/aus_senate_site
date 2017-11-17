$(document).ready(function() {
    $("form").submit(function(e) {
        var form = $(this);
        $.ajax({
            type: "POST",
            data: form.serialize(),
            success: function(response) {
                $("#content").html(response);
            },
            error: function(res, error) {
                // TODO: better errors once `unwraps` are removed from backend
                alert("Error. Try again?");
                location.reload();
            },
            // 5 minute timeout should be sufficient
            timeout: 5 * 60 * 1000,
        });

        $("#content").html("Please wait about a minute while the result computes...");

        return false;
    });

    $("#start_button").click(function() {
        let state = $("#state_select").val();
        location.href = state;
    });
});
