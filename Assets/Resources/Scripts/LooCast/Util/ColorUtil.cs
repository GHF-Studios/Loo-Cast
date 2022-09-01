using System;
using UnityEngine;

namespace LooCast.Util
{
    using LooCast.Mission;

    public static class ColorUtil
    {
        public static class RarityColors
        {
            public static readonly Color Common = new Color(1.0f, 1.0f, 1.0f, 1.0f);
            public static readonly Color Uncommon = new Color(0.1176471f, 1.0f, 0.0f, 1.0f);
            public static readonly Color Rare = new Color(0.0f, 0.4392157f, 0.8666667f, 1.0f);
            public static readonly Color Epic = new Color(0.6392157f, 0.2078431f, 0.9333333f, 1.0f);
            public static readonly Color Legendary = new Color(1.0f, 0.5019608f, 0.0f, 1.0f);

            public static Color GetMissionRarityColor(MissionRarity missionRarity)
            {
                switch (mission.MissionRarity)
                {
                    case MissionRarity.Common:
                        return ColorUtil.RarityColors.Common;
                        break;
                    case MissionRarity.Uncommon:
                        return ColorUtil.RarityColors.Uncommon;
                        break;
                    case MissionRarity.Rare:
                        return ColorUtil.RarityColors.Rare;
                        break;
                    case MissionRarity.Epic:
                        return ColorUtil.RarityColors.Epic;
                        break;
                    case MissionRarity.Legendary:
                        return ColorUtil.RarityColors.Legendary;
                        break;
                    default:
                        throw new NotImplementedException($"Mission Rarity '{mission.MissionRarity}' is not implemented!");
                }
            }
        }
    }
}
