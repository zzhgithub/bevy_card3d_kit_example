local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "NAAI-A-001",
    name = "维尔汀"
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name);
end
