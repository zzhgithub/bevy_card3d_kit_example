local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "S001-A-001",
    name = "APPLe",
    card_type = "Actor",
    attr = "STAR",
    race = "Awakened",
    cost = 1,
    ack = 1200,
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name, self.card_type, self.attr, self.race, self.cost, self.ack);
end
