wrk.method = "POST"
wrk.body = '{"interests":["coding","privacy","rust","gdpr"]}'
wrk.headers["Content-Type"] = "application/json"
wrk.headers["Cookie"] = "user_email="
