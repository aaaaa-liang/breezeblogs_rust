wrk.method = "POST"
wrk.body = '{"interests":["food", "fashion"]}'
wrk.headers["Content-Type"] = "application/json"
wrk.headers["Cookie"] = "user_email=test%40example.com"
