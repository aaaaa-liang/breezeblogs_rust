wrk.method = "POST"
wrk.body = '{"email":"newsletter@example.com"}'
wrk.headers["Content-Type"] = "application/json"
wrk.headers["Cookie"] = "user_email=test%40example.com"
