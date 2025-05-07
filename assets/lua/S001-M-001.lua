local ns = "entity_" .. entity.index;

_G[ns] = {
    id = "S001-M-001",
    name = "超酷太阳镜",
    card_type = "Meme",
    attr = "STAR",
    race = "NULL",
    cost = 3,
    ack = 0,
};

_G[ns].get_card_info = function(self)
    return CardInfo(self.id, self.name, self.card_type, self.attr, self.race, self.cost, self.ack);
end
