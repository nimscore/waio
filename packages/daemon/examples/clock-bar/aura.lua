-- Обновляем время каждую секунду
waio.timer.interval(1000, function()
    local time = waio.time.now()
    local date = waio.time.format("%Y-%m-%d")

    slint.set_property("time_text", time.str)
    slint.set_property("date_text", date)
end)

print("Clock bar aura loaded!")
