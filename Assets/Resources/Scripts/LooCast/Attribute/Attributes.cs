using System;
using System.IO;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute
{
    using LooCast.Attribute.Stat;
    using LooCast.Data;
    using LooCast.Util;
    using LooCast.Game;

    [CreateAssetMenu(fileName = "Attributes", menuName = "Data/Attribute/Attributes", order = 0)]
    public class Attributes : DynamicGameData
    {
        #region Fields
        public Stats Stats;

        public CharismaAttribute Charisma;
        public ConstitutionAttribute Constitution;
        public DefenseAttribute Defense;
        public DexterityAttribute Dexterity;
        public IntelligenceAttribute Intelligence;
        public LuckAttribute Luck;
        public PerceptionAttribute Perception;
        public StrengthAttribute Strength;
        public WillpowerAttribute Willpower;
        public WisdomAttribute Wisdom;
        #endregion

        #region Methods
        public void Cheat()
        {
            Charisma.Level.Value = Charisma.MaxLevel.Value;
            Constitution.Level.Value = Constitution.MaxLevel.Value;
            Defense.Level.Value = Defense.MaxLevel.Value;
            Dexterity.Level.Value = Dexterity.MaxLevel.Value;
            Intelligence.Level.Value = Intelligence.MaxLevel.Value;
            Luck.Level.Value = Luck.MaxLevel.Value;
            Perception.Level.Value = Perception.MaxLevel.Value;
            Strength.Level.Value = Strength.MaxLevel.Value;
            Willpower.Level.Value = Willpower.MaxLevel.Value;
            Wisdom.Level.Value = Wisdom.MaxLevel.Value;
        }

        public void Uncheat()
        {
            Charisma.Level.Value = 0;
            Constitution.Level.Value = 0;
            Defense.Level.Value = 0;
            Dexterity.Level.Value = 0;
            Intelligence.Level.Value = 0;
            Luck.Level.Value = 0;
            Perception.Level.Value = 0;
            Strength.Level.Value = 0;
            Willpower.Level.Value = 0;
            Wisdom.Level.Value = 0;
        }

        public Attribute GetAttribute(string attributeName)
        {
            switch (attributeName)
            {
                case "Charisma": return Charisma;
                case "Constitution": return Constitution;
                case "Defense": return Defense;
                case "Dexterity": return Dexterity;
                case "Intelligence": return Intelligence;
                case "Luck": return Luck;
                case "Perception": return Perception;
                case "Strength": return Strength;
                case "Willpower": return Willpower;
                case "Wisdom": return Wisdom;
                default: throw new ArgumentException("Invalid attribute name!");
            }
        }

        public override void Save()
        {
            SaveAttribute(Charisma);
            SaveAttribute(Constitution);
            SaveAttribute(Defense);
            SaveAttribute(Dexterity);
            SaveAttribute(Intelligence);
            SaveAttribute(Luck);
            SaveAttribute(Perception);
            SaveAttribute(Strength);
            SaveAttribute(Willpower);
            SaveAttribute(Wisdom);
        }

        public override void Load()
        {
            LoadAttribute(Charisma);
            LoadAttribute(Constitution);
            LoadAttribute(Defense);
            LoadAttribute(Dexterity);
            LoadAttribute(Intelligence);
            LoadAttribute(Luck);
            LoadAttribute(Perception);
            LoadAttribute(Strength);
            LoadAttribute(Willpower);
            LoadAttribute(Wisdom);
        }

        public override void LoadDefault()
        {
            LoadAttributeDefault(Charisma, new string[] { "Presence", "Charm", "Social" }, 0, 10, 0);
            LoadAttributeDefault(Constitution, new string[] { "Stamina", "Endurance", "Vitality", "Recovery" }, 0, 10, 0);
            LoadAttributeDefault(Defense, new string[] { "Resistance", "Fortitude", "Resilience" }, 0, 10, 0);
            LoadAttributeDefault(Dexterity, new string[] { "Agility", "Reflexes", "Quickness" }, 0, 10, 0);
            LoadAttributeDefault(Intelligence, new string[] { "Intellect", "Mind", "Knowledge" }, 0, 10, 0);
            LoadAttributeDefault(Luck, new string[] { "Fate", "Chance", "Fortune" }, 0, 10, 0);
            LoadAttributeDefault(Perception, new string[] { "Alertness", "Awareness", "Cautiousness" }, 0, 10, 0);
            LoadAttributeDefault(Strength, new string[] { "Body", "Might", "Brawn", "Power" }, 0, 10, 0);
            LoadAttributeDefault(Willpower, new string[] { "Sanity", "Personality", "Ego", "Resolve" }, 0, 10, 0);
            LoadAttributeDefault(Wisdom, new string[] { "Spirit", "Wits", "Psyche", "Sense" }, 0, 10, 0);
        }

        private void SaveAttribute(Attribute attribute)
        {
            string path = Path.Combine(Game.CurrentRelativeDataPath, "Attribute", $"{attribute.AttributeName}.dat");
            SerializationUtil.SaveData(new Attribute.DataContainer(attribute.Stats, attribute.Level, attribute.MaxLevel, attribute.ProposedLevelChange), path);
        }

        private void LoadAttribute(Attribute attribute)
        {
            string path = Path.Combine(Game.CurrentRelativeDataPath, "Attribute", $"{attribute.AttributeName}.dat");
            Attribute.DataContainer dataContainer = SerializationUtil.LoadData<Attribute.DataContainer>(path);
            attribute.Stats = dataContainer.GetStats(Stats);
            attribute.Level = dataContainer.GetLevel();
            attribute.MaxLevel = dataContainer.GetMaxLevel();
            attribute.ProposedLevelChange = dataContainer.GetProposedLevelChange();

            attribute.Level.OnValueChanged.AddListener(() =>
            {
                foreach (Stat.Stat stat in attribute.Stats)
                {
                    stat.MaxLevel.Value = attribute.Level.Value;
                }
            });
        }

        private void LoadAttributeDefault(Attribute attribute, string[] statNames, int level, int maxLevel, int proposedLevelChange)
        {
            Attribute.DataContainer dataContainer = new Attribute.DataContainer(statNames, level, maxLevel, proposedLevelChange);
            attribute.Stats = dataContainer.GetStats(Stats);
            attribute.Level = dataContainer.GetLevel();
            attribute.MaxLevel = dataContainer.GetMaxLevel();
            attribute.ProposedLevelChange = dataContainer.GetProposedLevelChange();

            attribute.Level.OnValueChanged.AddListener(() =>
            {
                foreach (Stat.Stat stat in attribute.Stats)
                {
                    stat.MaxLevel.Value = attribute.Level.Value;
                }
            });
        }
        #endregion
    }
}
