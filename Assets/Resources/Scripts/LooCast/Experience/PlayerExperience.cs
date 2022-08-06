using UnityEngine;

namespace LooCast.Experience
{
    using Data;
    using Data.Runtime;
    using Variable;
    using Attribute.Stat;
    using Currency;
    using Sound;

    public sealed class PlayerExperience : Experience
    {
        public PlayerExperienceData Data;
        public PlayerExperienceRuntimeData RuntimeData;
        public Stats Stats;
        public Coins Coins;
        public Tokens Tokens;

        private GameSoundHandler soundHandler;

        private void Start()
        {
            Initialize(Data);

            RuntimeData.CurrentExperience = new FloatComputedVariable(Data.BaseExperience.Value);
            RuntimeData.CurrentExperience.AddPermanentMultiplier(Stats.ExperienceMultiplier);
            RuntimeData.LevelExperienceMax = new FloatComputedVariable(Data.BaseLevelExperienceMax.Value);
            RuntimeData.CurrentLevel = new IntVariable(Data.BaseLevel.Value);

            soundHandler = FindObjectOfType<GameSoundHandler>();
        }

        public override void AddExperience(float xp)
        {
            Coins.Balance.Value += Mathf.RoundToInt(xp);
            RuntimeData.CurrentExperience.BaseValue += xp;

            UpdateLevelProgress(RuntimeData.CurrentExperience.Value);
        }

        protected override void UpdateLevelProgress(float overflowXP)
        {
            if (overflowXP == RuntimeData.LevelExperienceMax.Value)
            {
                IncreaseLevel();
                RuntimeData.CurrentExperience.BaseValue = 0;
                return;
            }

            if (overflowXP > RuntimeData.LevelExperienceMax.Value)
            {
                UpdateLevelProgress(overflowXP - RuntimeData.LevelExperienceMax.Value);
                IncreaseLevel();
                return;
            }

            if (overflowXP < RuntimeData.LevelExperienceMax.Value)
            {
                RuntimeData.CurrentExperience.BaseValue = overflowXP;
                return;
            }
        }

        protected override void IncreaseLevel()
        {
            RuntimeData.CurrentLevel.Value++;
            RuntimeData.LevelExperienceMax.BaseValue *= Stats.LevelExperienceMaxMultiplier;
            Tokens.Balance.Value++;
            soundHandler.SoundUpgrade();
        }
    } 
}