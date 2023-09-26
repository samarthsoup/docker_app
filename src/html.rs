pub const CONTAINERS: &str = r#"
<!doctype html>
<html lang="en">
<head>
</head>
<body style="background-color:#121212; font-family:helvetica;" link="ivory;" alink="cornsilk" vlink="ivory">
<p>
    <table style="color: cornsilk; padding: 10px;">
        <tr>
            <td colspan = "3" style = "font-size: 30px;">Active Containers</td>
        </tr>

        <tr style = "font-size: 20px;">
            <th>Name</th>
            <th>Image</th>
            <th>Status</th>
        </tr>
        {% for container in containers %}
            <tr style = "font-size: 20px;">
                <td style = "padding: 10px;">{{ container.Name }}</td>
                <td rowspan = "2" style = "padding: 10px;">{{ container.Image }}</td>
                <td rowspan = "2" style = "padding: 10px;">{{ container.Status }}</td>
            </tr>
            <tr style = "font-size: 12px;">
                <td style = "padding: 10px;">{{ container.Id }}</td>
            </tr>
        {% endfor %}
    </table>
</p>
</body>
</html>
"#;