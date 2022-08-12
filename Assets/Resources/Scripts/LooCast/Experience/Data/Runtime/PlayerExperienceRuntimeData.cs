using UnityEngine;

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
        }

        public FloatComputedVariable CurrentExperience;
        public FloatComputedVariable LevelExperienceMax;
        public IntVariable CurrentLevel;
    }
}
