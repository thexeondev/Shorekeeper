setTimeout(() => {
    const UiManager_1 = require("../Ui/UiManager");
    const UE = require("ue");
    const ControllerManagerBase_1 = require("../../Core/Framework/ControllerManagerBase");

    const UiText = UiManager_1.UiManager.GetViewByName("UidView").GetText(0);
    UiText.SetText("{PLAYER_USERNAME} - Reversed Rooms");
    UiText.SetColor(UE. Color.FromHex("{SELECTED_COLOR}"));
}, 10000);