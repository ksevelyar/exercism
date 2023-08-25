defmodule BinarySearchTree do
  @type bst_node :: %{data: any, left: bst_node | nil, right: bst_node | nil}

  @doc """
  Create a new Binary Search Tree with root's value as the given 'data'
  """
  @spec new(any) :: bst_node
  def new(data) do
    %{data: data, left: nil, right: nil}
  end

  @doc """
  Creates and inserts a node with its value as 'data' into the tree.
  """
  @spec insert(bst_node, any) :: bst_node
  def insert(%{data: tree_data, right: nil} = tree, data) when data > tree_data do
    %{tree | right: new(data)}
  end

  def insert(%{data: tree_data} = tree, data) when data > tree_data do
    %{tree | right: insert(tree.right, data)}
  end

  def insert(%{left: nil} = tree, data) do
    %{tree | left: new(data)}
  end

  def insert(tree, data) do
    %{tree | left: insert(tree.left, data)}
  end

  @doc """
  Traverses the Binary Search Tree in order and returns a list of each node's data.
  """
  @spec in_order(bst_node) :: [any]
  # TODO: push nodes to a queue
  def in_order(tree) do
    # Your implementation here
  end
end
