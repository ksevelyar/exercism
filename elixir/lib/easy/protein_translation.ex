defmodule ProteinTranslation do
  @doc """
  Given an RNA string, return a list of proteins specified by codons, in order.
  """

  @codons %{
    "UGU" => "Cysteine",
    "UGC" => "Cysteine",
    "UUA" => "Leucine",
    "UUG" => "Leucine",
    "AUG" => "Methionine",
    "UUU" => "Phenylalanine",
    "UUC" => "Phenylalanine",
    "UCU" => "Serine",
    "UCC" => "Serine",
    "UCA" => "Serine",
    "UCG" => "Serine",
    "UGG" => "Tryptophan",
    "UAU" => "Tyrosine",
    "UAC" => "Tyrosine",
    "UAA" => "STOP",
    "UAG" => "STOP",
    "UGA" => "STOP"
  }

  @spec of_rna(String.t()) :: {:ok, list(String.t())} | {:error, String.t()}
  def of_rna(rna), do: proteins(rna, [])

  defp proteins("", list), do: {:ok, Enum.reverse(list)}
  defp proteins(<<codon :: binary-size(3)>> <> _tail, list) when codon in ["UAA", "UAG", "UGA"] do
    proteins("", list)
  end

  defp proteins(<<codon :: binary-size(3)>> <> tail, list) when is_map_key(@codons, codon) do
    {:ok, protein} = of_codon(codon)
    proteins(tail, [protein | list])
  end

  defp proteins(_rna, _list), do: {:error, "invalid RNA"}

  @doc """
  Given a codon, return the corresponding protein
  """
  @spec of_codon(String.t()) :: {:ok, String.t()} | {:error, String.t()}
  def of_codon(codon) when is_map_key(@codons, codon), do: {:ok, @codons[codon]}
  def of_codon(_codon), do: {:error, "invalid codon"}
end
