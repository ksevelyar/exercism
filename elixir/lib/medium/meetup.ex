defmodule Meetup do
  @weekdays %{
    monday: 1,
    tuesday: 2,
    wednesday: 3,
    thursday: 4,
    friday: 5,
    saturday: 6,
    sunday: 7
  }
  @teenth 13..19

  def meetup(year, month, weekday, schedule) do
    beginning_of_month = Date.new!(year, month, 1)
    end_of_month = Date.end_of_month(beginning_of_month)
    month = Date.range(beginning_of_month, end_of_month)

    month |> match_weekdays(weekday) |> match_schedule(schedule)
  end

  defp match_weekdays(range, weekday) do
    weekday = @weekdays[weekday]

    Stream.filter(range, fn date -> Date.day_of_week(date) == weekday end)
  end

  defp match_schedule(range, :first), do: Enum.at(range, 0)
  defp match_schedule(range, :second), do: Enum.at(range, 1)
  defp match_schedule(range, :third), do: Enum.at(range, 2)
  defp match_schedule(range, :fourth), do: Enum.at(range, 3)

  defp match_schedule(range, :teenth) do
    Enum.find(range, fn date -> date.day in @teenth end)
  end

  defp match_schedule(range, :last) do
    range |> Enum.reverse() |> Enum.at(0)
  end
end
