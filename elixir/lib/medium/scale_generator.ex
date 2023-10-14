defmodule ScaleGenerator do
  @scale ~w(C C# D D# E F F# G G# A A# B)
  @flat_scale ~w(A Bb B C Db D Eb E F Gb G Ab)
  @flat_tonics ~w(F Bb Eb Ab Db Gb d g c f bb eb)

  def step(scale, tonic, step) do
    start_index = Enum.find_index(scale, fn x -> x == tonic end)

    Enum.at(scale, start_index + step(step))
  end

  defp step(step) do
    case step do
      "m" -> 1
      "M" -> 2
      "A" -> 3
    end
  end

  @doc """
  "C" should generate: ~w(C C# D D# E F F# G G# A A# B C)
  """
  @spec chromatic_scale(tonic :: String.t()) :: list(String.t())
  def chromatic_scale(tonic \\ "C"), do: shift_scale(@scale, tonic)

  @doc """
  "C" should generate: ~w(C Db D Eb E F Gb G Ab A Bb B C)
  """
  @spec flat_chromatic_scale(tonic :: String.t()) :: list(String.t())
  def flat_chromatic_scale(tonic \\ "C"), do: shift_scale(@flat_scale, tonic)

  defp shift_scale(scale, tonic) do
    tonic = String.capitalize(tonic)

    scale
    |> Stream.cycle()
    |> Stream.drop_while(&(&1 != tonic))
    |> Enum.take(length(scale) + 1)
  end

  @doc """
  For any of the following tonics, use the flat chromatic scale:
  F Bb Eb Ab Db Gb d g c f bb eb
  """
  @spec find_chromatic_scale(tonic :: String.t()) :: list(String.t())
  def find_chromatic_scale(tonic) when tonic in @flat_tonics, do: flat_chromatic_scale(tonic)
  def find_chromatic_scale(tonic), do: chromatic_scale(tonic)

  @doc """
  "MMmMMMm" -> C D E F G A B C
  """
  @spec scale(tonic :: String.t(), pattern :: String.t()) :: list(String.t())
  def scale(tonic, pattern) do
    scale = find_chromatic_scale(tonic)

    indexes =
      String.graphemes(pattern)
      |> Enum.reduce([0], fn step, [max_index | indexes] ->
        [step(step) + max_index, max_index | indexes]
      end)

    Enum.with_index(scale)
    |> Enum.filter(fn {_ch, ind} -> ind in indexes end)
    |> Enum.map(fn {ch, _ind} -> ch end)
  end
end
