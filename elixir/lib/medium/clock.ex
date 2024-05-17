defmodule Clock do
  @minutes_per_hour 60
  @hours_per_day 24
  @minutes_per_day @minutes_per_hour * @hours_per_day

  defstruct hour: 0, minute: 0

  @doc """
  Returns a clock that can be represented as a string:

      iex> Clock.new(8, 9) |> to_string
      "08:09"
  """
  @spec new(integer, integer) :: Clock
  def new(hour, minute) do
    total_minute = rem(hour * @minutes_per_hour + minute, @minutes_per_day)
    total_minute = if total_minute < 0, do: @minutes_per_day + total_minute, else: total_minute

    %Clock{
      hour: div(total_minute, @minutes_per_hour),
      minute: rem(total_minute, @minutes_per_hour)
    }
  end

  @doc """
  Adds two clock times:

      iex> Clock.new(10, 0) |> Clock.add(3) |> to_string
      "10:03"
  """
  @spec add(Clock, integer) :: Clock
  def add(%Clock{hour: hour, minute: minute}, add_minute) do
    new(hour, minute + add_minute)
  end
end

defimpl String.Chars, for: Clock do
  def to_string(%Clock{hour: hour, minute: minute}) do
    hour = String.pad_leading(Integer.to_string(hour), 2, "0")
    minute = String.pad_leading(Integer.to_string(minute), 2, "0")

    "#{hour}:#{minute}"
  end
end
