local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "S001-T-001",
    name = "一些微小的工",
    card_type = "Arcane",
    attr = "STAR",
    race = "NULL",
    cost = 0,
    ack = 0,
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name, self.card_type, self.attr, self.race, self.cost, self.ack);
end
