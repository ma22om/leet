class Nite
  attr_reader :nodes
  class Node
    attr_accessor :char, :col, :row, :disc, :parent
    attr_reader :adjies
    def initialize(char, col, row)
      @char = char
      @col = col
      @row = row
      @adjies = [] # adjacencies
      @disc = false
      @parent = nil
    end
    def add(value)
      @adjies << value
    end
  end

  def add(value)
    @nodes << value
  end

  def _read
    # assign each coord. to a Node-object
    n = gets.to_i
    nodes = Array.new
    n.times do |x|
      temp = gets.strip.to_s
      temp.size.times do |y|
        node = Node.new(temp[y], x, y)
        nodes << node
      end
    end
    
    
    nodes
  end

  private def adjacencify(x, y)
    [[1,2], [1,-2], [-1,2], [-1,-2], [2,1], [2,-1], [-2,1], [-2,-1]].map do |dx, dy|
      @nodes[[x+dx,y+dy]]
    end.compact

  def _jump(nodes)

    nodes = _adjacencify(nodes)

    q = Queue.new
    root = nodes[0]
    root.disc = true
    q << root
    
    while !q.empty? do
      v = q.pop
      if v.char == 'K'
        steps = 0
        until v.parent == nil do
          if !v.nil?
          end
          steps += 1
          v.parent = v.parent.parent
        end
        return steps
      end
      v.adjies.each do |w|
        if w.disc == false
          w.parent = v
          w.disc = true
          q << w
        end
      end
    end
    return -1

  end

  def find_path
    _jump(_read)
  end

end

nite = Nite.new
puts nite.find_path
