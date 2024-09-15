const UE = require("ue");
const { CharacterDitherEffectController } = require("../NewWorld/Character/Common/Component/Effect/CharacterDitherEffectController");

CharacterDitherEffectController.prototype.SetDitherEffect = function(t, i = 3, s = true) { };

CharacterDitherEffectController.prototype.EnterAppearEffect = function(t = 1, i = 3, s = true) {
    this.SetHiddenInGame(false, true);
};

CharacterDitherEffectController.prototype.EnterDisappearEffect = function(t = 1, i = 3, s = true) {
    this.SetHiddenInGame(true, true);
};