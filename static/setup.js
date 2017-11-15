$(document).ready(function() {
    $("form").submit(function(e) {
        var form = $(this);
        $.ajax({
            type: "POST",
            data: form.serialize(),
            success: function(response) {
                $("body").html(response);
            }
        });

        form.html("Please wait about a minute while the result computes...");

        return false;
    });
});
