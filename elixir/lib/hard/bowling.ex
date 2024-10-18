defmodule Bowling do
  @doc """
    Creates a new game of bowling that can be used to store the results of
    the game
  """

  defstruct frames: []

  @spec start() :: any
  def start, do: %Bowling{}

  @doc """
    Records the number of pins knocked down on a single roll. Returns `any`
    unless there is something wrong with the given number of pins, in which
    case it returns a helpful error tuple.
  """

  @spec roll(any, integer) :: {:ok, any} | {:error, String.t()}
  def roll(_, roll) when roll < 0, do: {:error, "Negative roll is invalid"}
  def roll(_, roll) when roll > 10, do: {:error, "Pin count exceeds pins on the lane"}

  def roll(%Bowling{frames: frames}, roll) do
    cond do
      complete?(frames) ->
        {:error, "Cannot roll after game is over"}

      last_frame_closed?(frames) ->
        {:ok, %Bowling{frames: [{roll, nil, nil} | frames]}}

      true ->
        [frame | rest] = frames

        with {:ok, frame} <- add_roll_to_frame(roll, frame) do
          {:ok, %Bowling{frames: [frame | rest]}}
        end
    end
  end

  @doc """
    Returns the score of a given game of bowling if the game is complete.
    If the game isn't complete, it returns a helpful error tuple.
  """

  @spec score(any) :: {:ok, integer} | {:error, String.t()}
  def score(game) do
    if complete?(game.frames) do
      frames = Enum.reverse(game.frames)

      score =
        frames
        |> Stream.with_index()
        |> Stream.map(&score_frame(&1, frames))
        |> Enum.sum()

      {:ok, score}
    else
      {:error, "Score cannot be taken until the end of the game"}
    end
  end

  defp score_frame({{p1, p2, p3}, index}, frames) do
    frame_pins = Enum.reject([p1, p2, p3], &is_nil/1) |> Enum.sum()

    case {length(Enum.reject([p1, p2, p3], &is_nil/1)), frame_pins} do
      {1, 10} -> strike_score(frames, index)
      {_, 10} -> spare_score(frames, index)
      _ -> frame_pins
    end
  end

  defp strike_score(frames, index) do
    next_frame = Enum.at(frames, index + 1, {0, 0, 0})
    {n1, n2, _n3} = next_frame

    if n2 == nil do
      {nn1, _nn2, _nn3} = Enum.at(frames, index + 2, {0, 0, 0})
      n1 + nn1 + 10
    else
      n1 + n2 + 10
    end
  end

  defp spare_score(frames, index) do
    {n1, _n2, _n3} = Enum.at(frames, index + 1, {0, 0, 0})
    n1 + 10
  end

  defp complete?(frames) do
    last_frame? = length(frames) == 10

    last_frame_closed? =
      case Enum.at(frames, 0) do
        {_a, nil, nil} -> false
        {a, b, nil} when a + b == 10 -> false
        {a, _b, nil} -> a != 10
        _ -> true
      end

    last_frame? && last_frame_closed?
  end

  defp add_roll_to_frame(roll, frame) do
    case frame do
      {pins0, nil, nil} when pins0 + roll > 10 and pins0 != 10 ->
        {:error, "Pin count exceeds pins on the lane"}

      {pins0, nil, nil} ->
        {:ok, {pins0, roll, nil}}

      {10, 10, nil} ->
        {:ok, {10, 10, roll}}

      {10, pins1, nil} when pins1 + roll > 10 ->
        {:error, "Pin count exceeds pins on the lane"}

      {pins0, pins1, nil} ->
        {:ok, {pins0, pins1, roll}}
    end
  end

  defp last_frame_closed?([]), do: true

  defp last_frame_closed?([frame | _rest] = frames) do
    case frame do
      {10, _, _} ->
        length(frames) != 10

      {pins0, pins1, _} when pins1 != nil ->
        length(frames) != 10 || pins0 + pins1 != 10

      _ ->
        false
    end
  end
end
