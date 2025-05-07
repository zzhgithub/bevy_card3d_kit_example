local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "NAAI-A-001",
    name = "维尔汀",
    card_type = "Actor",
    attr = "INTELLECT",
    race = "Human",
    cost = 0,
    ack = 0,
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name, self.card_type, self.attr, self.race, self.cost, self.ack);
end
