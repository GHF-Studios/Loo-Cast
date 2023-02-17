using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using LooCast.Data;
    using LooCast.Util;

    [CreateAssetMenu(fileName = "Stats", menuName = "Data/Attribute/Stat/Stats", order = 0)]
    public class Stats : DynamicGameData
    {
        #region Properties
        public float MovementSpeedMultiplier { get { return Agility.MovementSpeedMultiplier; } }
        //public float UNDEFINED { get { return Alertness.UNDEFINED; } }
        //public float UNDEFINED { get { return Awareness.UNDEFINED; } }
        public float EnergyMultiplier { get { return Body.EnergyMultiplier; } }
        public int ArmorPenetrationIncrease { get { return Brawn.ArmorPenetrationIncrease; } }
        //public float UNDEFINED { get { return Cautiousness.UNDEFINED; } }
        public float RandomChanceMultiplier { get { return Chance.RandomChanceMultiplier; } }
        //public float UNDEFINED { get { return Charm.UNDEFINED; } }
        public float DamageReflection { get { return Ego.DamageReflection; } }
        public float EnergyRegenerationMultiplier { get { return Endurance.EnergyRegenerationMultiplier; } }
        public float NegativeEventChanceMultiplier { get { return Fate.NegativeEventChanceMultiplier; } }
        public float EnergyConsumptionMultiplier { get { return Fortitude.EnergyConsumptionMultiplier; } }
        public float PositiveEventChanceMultiplier { get { return Fortune.PositiveEventChanceMultiplier; } }
        public float ExperienceMultiplier { get { return Intellect.ExperienceMultiplier; } }
        public float LevelExperienceMaxMultiplier { get { return Knowledge.LevelExperienceMaxMultiplier; } }
        public float DamageMultiplier { get { return Might.DamageMultiplier; } }
        public float RangeMultiplier { get { return Mind.RangeMultiplier; } }
        public float ProjectileSpeedMultiplier { get { return Personality.ProjectileSpeedMultiplier; } }
        public float KnockbackMultiplier { get { return Power.KnockbackMultiplier; } }
        //public float UNDEFINED { get { return Presence.UNDEFINED; } }
        //public float UNDEFINED { get { return Psyche.UNDEFINED; } }
        public float AttackDelayMultiplier { get { return Quickness.AttackDelayMultiplier; } }
        public float HealthRegenrationMultiplier { get { return Recovery.HealthRegenrationMultiplier; } }
        public float ConsecutiveProjectileDelayMultiplier { get { return Reflexes.ConsecutiveProjectileDelayMultiplier; } }
        public float KnockbackResistanceMultiplier { get { return Resilience.KnockbackResistanceMultiplier; } }
        public int DefenseIncrease { get { return Resistance.DefenseIncrease; } }
        public int PiercingIncrease { get { return Resolve.PiercingIncrease; } }
        public float ProjectileSizeMultiplier { get { return Sanity.ProjectileSizeMultiplier; } }
        //public float UNDEFINED { get { return Sense.UNDEFINED; } }
        //public float UNDEFINED { get { return Social.UNDEFINED; } }
        //public float UNDEFINED { get { return Spirit.UNDEFINED; } }
        public float DurationMultiplier { get { return Stamina.DurationMultiplier; } }
        public float HealthMultiplier { get { return Vitality.HealthMultiplier; } }
        //public float UNDEFINED { get { return Wits.UNDEFINED; } }
        #endregion

        #region Fields
        public Attributes Attributes;

        public AgilityStat Agility;
        public AlertnessStat Alertness;
        public AwarenessStat Awareness;
        public BodyStat Body;
        public BrawnStat Brawn;
        public CautiousnessStat Cautiousness;
        public ChanceStat Chance;
        public CharmStat Charm;
        public EgoStat Ego;
        public EnduranceStat Endurance;
        public FateStat Fate;
        public FortitudeStat Fortitude;
        public FortuneStat Fortune;
        public IntellectStat Intellect;
        public KnowledgeStat Knowledge;
        public MightStat Might;
        public MindStat Mind;
        public PersonalityStat Personality;
        public PowerStat Power;
        public PresenceStat Presence;
        public PsycheStat Psyche;
        public QuicknessStat Quickness;
        public RecoveryStat Recovery;
        public ReflexesStat Reflexes;
        public ResilienceStat Resilience;
        public ResistanceStat Resistance;
        public ResolveStat Resolve;
        public SanityStat Sanity;
        public SenseStat Sense;
        public SocialStat Social;
        public SpiritStat Spirit;
        public StaminaStat Stamina;
        public VitalityStat Vitality;
        public WitsStat Wits;
        #endregion

        #region Methods
        public void Cheat()
        {
            Agility.Level.Value = Agility.MaxLevel.Value;
            Alertness.Level.Value = Alertness.MaxLevel.Value;
            Awareness.Level.Value = Awareness.MaxLevel.Value;
            Body.Level.Value = Body.MaxLevel.Value;
            Brawn.Level.Value = Brawn.MaxLevel.Value;
            Cautiousness.Level.Value = Cautiousness.MaxLevel.Value;
            Chance.Level.Value = Chance.MaxLevel.Value;
            Charm.Level.Value = Charm.MaxLevel.Value;
            Ego.Level.Value = Ego.MaxLevel.Value;
            Endurance.Level.Value = Endurance.MaxLevel.Value;
            Fate.Level.Value = Fate.MaxLevel.Value;
            Fortitude.Level.Value = Fortitude.MaxLevel.Value;
            Fortune.Level.Value = Fortune.MaxLevel.Value;
            Intellect.Level.Value = Intellect.MaxLevel.Value;
            Knowledge.Level.Value = Knowledge.MaxLevel.Value;
            Might.Level.Value = Might.MaxLevel.Value;
            Mind.Level.Value = Mind.MaxLevel.Value;
            Personality.Level.Value = Personality.MaxLevel.Value;
            Power.Level.Value = Power.MaxLevel.Value;
            Presence.Level.Value = Presence.MaxLevel.Value;
            Psyche.Level.Value = Psyche.MaxLevel.Value;
            Quickness.Level.Value = Quickness.MaxLevel.Value;
            Recovery.Level.Value = Recovery.MaxLevel.Value;
            Reflexes.Level.Value = Reflexes.MaxLevel.Value;
            Resilience.Level.Value = Resilience.MaxLevel.Value;
            Resistance.Level.Value = Resistance.MaxLevel.Value;
            Resolve.Level.Value = Resolve.MaxLevel.Value;
            Sanity.Level.Value = Sanity.MaxLevel.Value;
            Sense.Level.Value = Sense.MaxLevel.Value;
            Social.Level.Value = Social.MaxLevel.Value;
            Spirit.Level.Value = Spirit.MaxLevel.Value;
            Stamina.Level.Value = Stamina.MaxLevel.Value;
            Vitality.Level.Value = Vitality.MaxLevel.Value;
            Wits.Level.Value = Wits.MaxLevel.Value;
        }

        public void Uncheat()
        {
            Agility.Level.Value = 0;
            Alertness.Level.Value = 0;
            Awareness.Level.Value = 0;
            Body.Level.Value = 0;
            Brawn.Level.Value = 0;
            Cautiousness.Level.Value = 0;
            Chance.Level.Value = 0;
            Charm.Level.Value = 0;
            Ego.Level.Value = 0;
            Endurance.Level.Value = 0;
            Fate.Level.Value = 0;
            Fortitude.Level.Value = 0;
            Fortune.Level.Value = 0;
            Intellect.Level.Value = 0;
            Knowledge.Level.Value = 0;
            Might.Level.Value = 0;
            Mind.Level.Value = 0;
            Personality.Level.Value = 0;
            Power.Level.Value = 0;
            Presence.Level.Value = 0;
            Psyche.Level.Value = 0;
            Quickness.Level.Value = 0;
            Recovery.Level.Value = 0;
            Reflexes.Level.Value = 0;
            Resilience.Level.Value = 0;
            Resistance.Level.Value = 0;
            Resolve.Level.Value = 0;
            Sanity.Level.Value = 0;
            Sense.Level.Value = 0;
            Social.Level.Value = 0;
            Spirit.Level.Value = 0;
            Stamina.Level.Value = 0;
            Vitality.Level.Value = 0;
            Wits.Level.Value = 0;
        }

        public Stat GetStat(string statName)
        {
            switch (statName)
            {
                case "Agility": return Agility;
                case "Alertness": return Alertness;
                case "Awareness": return Awareness;
                case "Body": return Body;
                case "Brawn": return Brawn;
                case "Cautiousness": return Cautiousness;
                case "Chance": return Chance;
                case "Charm": return Charm;
                case "Ego": return Ego;
                case "Endurance": return Endurance;
                case "Fate": return Fate;
                case "Fortitude": return Fortitude;
                case "Fortune": return Fortune;
                case "Intellect": return Intellect;
                case "Knowledge": return Knowledge;
                case "Might": return Might;
                case "Mind": return Mind;
                case "Personality": return Personality;
                case "Power": return Power;
                case "Presence": return Presence;
                case "Psyche": return Psyche;
                case "Quickness": return Quickness;
                case "Recovery": return Recovery;
                case "Reflexes": return Reflexes;
                case "Resilience": return Resilience;
                case "Resistance": return Resistance;
                case "Resolve": return Resolve;
                case "Sanity": return Sanity;
                case "Sense": return Sense;
                case "Social": return Social;
                case "Spirit": return Spirit;
                case "Stamina": return Stamina;
                case "Vitality": return Vitality;
                case "Wits": return Wits;
                default: throw new ArgumentException("Invalid stat namespaceName!");
            }
        }

        public override void Save()
        {
            SaveStat(Agility);
            SaveStat(Alertness);
            SaveStat(Awareness);
            SaveStat(Body);
            SaveStat(Brawn);
            SaveStat(Cautiousness);
            SaveStat(Chance);
            SaveStat(Charm);
            SaveStat(Ego);
            SaveStat(Endurance);
            SaveStat(Fate);
            SaveStat(Fortitude);
            SaveStat(Fortune);
            SaveStat(Intellect);
            SaveStat(Knowledge);
            SaveStat(Might);
            SaveStat(Mind);
            SaveStat(Personality);
            SaveStat(Power);
            SaveStat(Presence);
            SaveStat(Psyche);
            SaveStat(Quickness);
            SaveStat(Recovery);
            SaveStat(Reflexes);
            SaveStat(Resilience);
            SaveStat(Resistance);
            SaveStat(Resolve);
            SaveStat(Sanity);
            SaveStat(Sense);
            SaveStat(Social);
            SaveStat(Spirit);
            SaveStat(Stamina);
            SaveStat(Vitality);
            SaveStat(Wits);
        }

        public override void Load()
        {
            LoadStat(Agility);
            LoadStat(Alertness);
            LoadStat(Awareness);
            LoadStat(Body);
            LoadStat(Brawn);
            LoadStat(Cautiousness);
            LoadStat(Chance);
            LoadStat(Charm);
            LoadStat(Ego);
            LoadStat(Endurance);
            LoadStat(Fate);
            LoadStat(Fortitude);
            LoadStat(Fortune);
            LoadStat(Intellect);
            LoadStat(Knowledge);
            LoadStat(Might);
            LoadStat(Mind);
            LoadStat(Personality);
            LoadStat(Power);
            LoadStat(Presence);
            LoadStat(Psyche);
            LoadStat(Quickness);
            LoadStat(Recovery);
            LoadStat(Reflexes);
            LoadStat(Resilience);
            LoadStat(Resistance);
            LoadStat(Resolve);
            LoadStat(Sanity);
            LoadStat(Sense);
            LoadStat(Social);
            LoadStat(Spirit);
            LoadStat(Stamina);
            LoadStat(Vitality);
            LoadStat(Wits);
        }

        public override void LoadDefault()
        {
            LoadStatDefault(Agility, "Dexterity", 0, 0, 0);
            LoadStatDefault(Alertness, "Perception", 0, 0, 0);
            LoadStatDefault(Awareness, "Perception", 0, 0, 0);
            LoadStatDefault(Body, "Strength", 0, 0, 0);
            LoadStatDefault(Brawn, "Strength", 0, 0, 0);
            LoadStatDefault(Cautiousness, "Perception", 0, 0, 0);
            LoadStatDefault(Chance, "Luck", 0, 0, 0);
            LoadStatDefault(Charm, "Charisma", 0, 0, 0);
            LoadStatDefault(Ego, "Willpower", 0, 0, 0);
            LoadStatDefault(Endurance, "Constitution", 0, 0, 0);
            LoadStatDefault(Fate, "Luck", 0, 0, 0);
            LoadStatDefault(Fortitude, "Defense", 0, 0, 0);
            LoadStatDefault(Fortune, "Luck", 0, 0, 0);
            LoadStatDefault(Intellect, "Intelligence", 0, 0, 0);
            LoadStatDefault(Knowledge, "Intelligence", 0, 0, 0);
            LoadStatDefault(Might, "Strength", 0, 0, 0);
            LoadStatDefault(Mind, "Intelligence", 0, 0, 0);
            LoadStatDefault(Personality, "Willpower", 0, 0, 0);
            LoadStatDefault(Power, "Strength", 0, 0, 0);
            LoadStatDefault(Presence, "Charisma", 0, 0, 0);
            LoadStatDefault(Psyche, "Wisdom", 0, 0, 0);
            LoadStatDefault(Quickness, "Dexterity", 0, 0, 0);
            LoadStatDefault(Recovery, "Constitution", 0, 0, 0);
            LoadStatDefault(Reflexes, "Dexterity", 0, 0, 0);
            LoadStatDefault(Resilience, "Defense", 0, 0, 0);
            LoadStatDefault(Resistance, "Defense", 0, 0, 0);
            LoadStatDefault(Resolve, "Willpower", 0, 0, 0);
            LoadStatDefault(Sanity, "Willpower", 0, 0, 0);
            LoadStatDefault(Sense, "Wisdom", 0, 0, 0);
            LoadStatDefault(Social, "Charisma", 0, 0, 0);
            LoadStatDefault(Spirit, "Wisdom", 0, 0, 0);
            LoadStatDefault(Stamina, "Constitution", 0, 0, 0);
            LoadStatDefault(Vitality, "Constitution", 0, 0, 0);
            LoadStatDefault(Wits, "Wisdom", 0, 0, 0);
        }

        private void SaveStat(Stat stat)
        {
            SerializationUtil.SaveData(new Stat.DataContainer(stat.Attribute, stat.Level, stat.MaxLevel, stat.ProposedLevelChange), $"Attribute/Stat/{stat.StatName}.dat");
        }

        private void LoadStat(Stat stat)
        {
            Stat.DataContainer dataContainer = SerializationUtil.LoadData<Stat.DataContainer>($"Attribute/Stat/{stat.StatName}.dat");
            stat.Attribute = dataContainer.GetAttribute(Attributes);
            stat.Level = dataContainer.GetLevel();
            stat.MaxLevel = dataContainer.GetMaxLevel();
            stat.ProposedLevelChange = dataContainer.GetProposedLevelChange();
        }

        private void LoadStatDefault(Stat stat, string attributeName, int level, int maxLevel, int proposedLevelChange)
        {
            Stat.DataContainer dataContainer = new Stat.DataContainer(attributeName, 0, 0, 0);
            stat.Attribute = dataContainer.GetAttribute(Attributes);
            stat.Level = dataContainer.GetLevel();
            stat.MaxLevel = dataContainer.GetMaxLevel();
            stat.ProposedLevelChange = dataContainer.GetProposedLevelChange();
        }
        #endregion
    }
}
