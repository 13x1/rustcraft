while true do
    http.request("http://localhost:9999/script/[SCRIPT]")

    local done = false;
    while not done do
        local ev, _, text = os.pullEvent()
        if ev == "http_success" then
            done = true
            term.clear()
            term.setCursorPos(1, 1)
            local script = text.readAll()
            local func, parse_err = loadstring(script)
            if func then
                local ok, err = pcall(func)
                if not ok then
                    print("Error running script: " .. err)
                end
            else
                print("Error loading script: " .. parse_err)
            end
        elseif ev == "http_failure" then
            done = true
            print("Failed to connect to server")
        end
    end
end