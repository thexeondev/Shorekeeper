const UE = require("ue"),
    Info_1 = require("../../../Core/Common/Info"),
    MathUtils_1 = require("../../../Core/Utils/MathUtils"),
    EventDefine_1 = require("../../Common/Event/EventDefine"),
    EventSystem_1 = require("../../Common/Event/EventSystem"),
    UiControllerBase_1 = require("../../Ui/Base/UiControllerBase"),
    UiLayerType_1 = require("../../Ui/Define/UiLayerType"),
    UiLayer_1 = require("../../Ui/UiLayer");

var _a = require('../Module/WaterMask/WaterMaskController').WaterMaskView;
_a.LOo = 0.15;
_a.yOo = 700;
_a.IOo = 700;
_a.vOo = function () {
    void 0 !== _a.SOo && _a.EOo();
    var e = UiLayer_1.UiLayer.GetLayerRootUiItem(UiLayerType_1.ELayerType.WaterMask),
        t = (_a.SOo = UE.KuroActorManager.SpawnActor(Info_1.Info.World, UE.UIContainerActor.StaticClass(),
            MathUtils_1.MathUtils.DefaultTransform, void 0), _a.SOo.RootComponent),
        e = (t.SetDisplayName("WaterMaskContainer"), UE.KuroStaticLibrary.SetActorPermanent(_a.SOo, !0, !0), _a.SOo
            .K2_AttachRootComponentTo(e), t.GetRootCanvas().GetOwner().RootComponent),
        i = e.widget.width % _a.yOo / 2,
        r = e.widget.height % _a.IOo / 2,
        n = e.widget.width / 2,
        _ = e.widget.height / 2,
        s = Math.ceil(e.widget.width / _a.yOo),
        o = Math.ceil(e.widget.height / _a.IOo),
        v = "NCSO @ discord.gg/reversedrooms";
    for (let a = 0; a < s; a++)
        for (let e = 0; e < o; e++) {
            var E = UE.KuroActorManager.SpawnActor(Info_1.Info.World, UE.UITextActor.StaticClass(), MathUtils_1
                    .MathUtils.DefaultTransform, void 0),
                U = E.RootComponent,
                U = (E.K2_AttachRootComponentTo(t), U.SetDisplayName("WaterMaskText"), E.GetComponentByClass(UE
                    .UIText.StaticClass()));
            U.SetFontSize(_a.vFt), U.SetOverflowType(0), U.SetAlpha(_a.LOo), U.SetFont(UE.LGUIFontData
                .GetDefaultFont()), U.SetText(v), U.SetUIRelativeLocation(new UE.Vector(a * _a.yOo - n + i, e *
                _a.IOo - _ + r, 0)), U.SetUIRelativeRotation(new UE.Rotator(0, _a.TOo, 0)), UE.KuroStaticLibrary
                .SetActorPermanent(E, !0, !0)
        }
};
_a.vOo();