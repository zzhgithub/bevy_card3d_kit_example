local ns = "entity_" .. entity.index;

_G[ns] = {
};

_G[ns].get_card_info = function()
    return CardInfo("NAAI-A-001", "维尔汀");
end
