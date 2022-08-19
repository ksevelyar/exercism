defmodule Lasagna do
  def expected_minutes_in_oven, do: 40

  def remaining_minutes_in_oven(passed), do: expected_minutes_in_oven() - passed

  def preparation_time_in_minutes(layers), do: layers * 2

  def total_time_in_minutes(layers, passed), do: preparation_time_in_minutes(layers) + passed

  def alarm, do: "Ding!"
end
