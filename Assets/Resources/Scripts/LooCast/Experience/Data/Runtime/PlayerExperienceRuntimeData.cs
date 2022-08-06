using UnityEngine;

namespace LooCast.Experience.Data.Runtime
{
    using LooCast.Variable;

    [CreateAssetMenu(fileName = "PlayerExperienceRuntimeData", menuName = "Data/Experience/PlayerExperienceRuntimeData", order = 0)]
    public sealed class PlayerExperienceRuntimeData : ScriptableObject
    {
        public FloatComputedVariable CurrentExperience;
        public FloatComputedVariable LevelExperienceMax;
        public IntVariable CurrentLevel;
    }
}
