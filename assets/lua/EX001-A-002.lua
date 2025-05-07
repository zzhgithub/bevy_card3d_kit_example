local ns = "entity_" .. entity.index;

_G[ns] = {
};

_G[ns].get_card_info = function()
    return CardInfo("EX001-A-002", "门");
end
