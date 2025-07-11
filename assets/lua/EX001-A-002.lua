local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "EX001-A-002",
    name = "门",
    card_type = "Actor",
    attr = "INTELLECT",
    race = "Arcanist",
    cost = 0,
    ack = 0,
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name, self.card_type, self.attr, self.race, self.cost, self.ack);
end
