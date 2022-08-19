defmodule Easy.Darts do
  @inner %{radius: 1, score: 10}
  @middle %{radius: 5, score: 5}
  @outer %{radius: 10, score: 1}

  defp distance_from_center(x, y), do: :math.sqrt(x * x + y * y)

  defp score_by_distance(distance) when distance <= @inner.radius, do: @inner.score
  defp score_by_distance(distance) when distance <= @middle.radius, do: @middle.score
  defp score_by_distance(distance) when distance <= @outer.radius, do: @outer.score
  defp score_by_distance(_), do: 0

  def score({x, y}), do: distance_from_center(x, y) |> score_by_distance
end
