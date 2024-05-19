defmodule FoodChain do
  @doc """
  Generate consecutive verses of the song 'I Know an Old Lady Who Swallowed a Fly'.
  """
  @spec recite(start :: integer, stop :: integer) :: String.t()
  def recite(start, stop) do
    start..stop |> Enum.map(&intro/1) |> Enum.join("\n")
  end

  defp intro(1) do
    """
    I know an old lady who swallowed a fly.
    #{verse(1)}
    """
  end

  defp intro(2) do
    """
    I know an old lady who swallowed a spider.
    It wriggled and jiggled and tickled inside her.
    #{verse(2)}
    """
  end

  defp intro(3) do
    """
    I know an old lady who swallowed a bird.
    How absurd to swallow a bird!
    #{verse(3)}
    """
  end

  defp intro(4) do
    """
    I know an old lady who swallowed a cat.
    Imagine that, to swallow a cat!
    #{verse(4)}
    """
  end

  defp intro(5) do
    """
    I know an old lady who swallowed a dog.
    What a hog, to swallow a dog!
    #{verse(5)}
    """
  end

  defp intro(6) do
    """
    I know an old lady who swallowed a goat.
    Just opened her throat and swallowed a goat!
    #{verse(6)}
    """
  end

  defp intro(7) do
    """
    I know an old lady who swallowed a cow.
    I don't know how she swallowed a cow!
    #{verse(7)}
    """
  end

  defp intro(8) do
    """
    I know an old lady who swallowed a horse.
    She's dead, of course!
    """
  end

  defp verse(1) do
    "I don't know why she swallowed the fly. Perhaps she'll die."
  end

  defp verse(2) do
    "She swallowed the spider to catch the fly.\n#{verse(1)}"
  end

  defp verse(3) do
    "She swallowed the bird to catch the spider that wriggled and jiggled and tickled inside her.\n#{verse(2)}"
  end

  defp verse(4) do
    "She swallowed the cat to catch the bird.\n#{verse(3)}"
  end

  defp verse(5) do
    "She swallowed the dog to catch the cat.\n#{verse(4)}"
  end

  defp verse(6) do
    "She swallowed the goat to catch the dog.\n#{verse(5)}"
  end

  defp verse(7) do
    "She swallowed the cow to catch the goat.\n#{verse(6)}"
  end
end
