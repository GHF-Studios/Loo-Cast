using UnityEngine;

namespace LooCast.Experience.Data
{
    using LooCast.Data;

    public abstract class ExperienceData : ScriptableObject
    {
        public FloatDataReference BaseExperience;
        public FloatDataReference BaseLevelExperienceMax;
        public IntDataReference BaseLevel;
    } 
}