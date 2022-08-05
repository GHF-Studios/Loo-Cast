using UnityEngine;

namespace LooCast.Experience.Data.Runtime
{
    [CreateAssetMenu(fileName = "PlayerExperienceRuntimeData", menuName = "Data/Experience/PlayerExperienceRuntimeData", order = 0)]
    public sealed class PlayerExperienceRuntimeData : ScriptableObject
    {
        public float CurrentExperience;
        public float LevelExperienceMax;
        public int CurrentLevel;
    }
}
