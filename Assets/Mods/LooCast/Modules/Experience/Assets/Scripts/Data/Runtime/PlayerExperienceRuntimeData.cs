using UnityEngine;
using Steamworks;

namespace LooCast.Experience.Data.Runtime
{
    using LooCast.Variable;
    using LooCast.Attribute.Stat;

    [CreateAssetMenu(fileName = "PlayerExperienceRuntimeData", menuName = "Data/Experience/PlayerExperienceRuntimeData", order = 0)]
    public class PlayerExperienceRuntimeData : ScriptableObject
    {
        public Stats Stats;

        public void Initialize(PlayerExperienceData data)
        {
            CurrentExperience = new FloatComputedVariable(data.BaseExperience.Value);
            CurrentExperience.AddPermanentMultiplier(Stats.ExperienceMultiplier);
            LevelExperienceMax = new FloatComputedVariable(data.BaseLevelExperienceMax.Value);
            CurrentLevel = new IntVariable(data.BaseLevel.Value);

            CurrentLevel.OnValueChanged.AddListener(() =>
            {
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_xplvl", out int highscore_xplvl);
                    if (CurrentLevel.Value > highscore_xplvl)
                    {
                        SteamUserStats.SetStat("highscore_xplvl", CurrentLevel.Value);
                    }
                    if (CurrentLevel.Value >= 69)
                    {
                        SteamUserStats.GetAchievement("The_Even_Funnier_Number", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Even_Funnier_Number");
                        }
                    }
                    SteamUserStats.StoreStats();
                }
            });
        }

        public FloatComputedVariable CurrentExperience;
        public FloatComputedVariable LevelExperienceMax;
        public IntVariable CurrentLevel;
    }
}
