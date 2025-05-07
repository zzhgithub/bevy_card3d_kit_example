local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "EX001-A-002",
    name = "门"
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name);
end
