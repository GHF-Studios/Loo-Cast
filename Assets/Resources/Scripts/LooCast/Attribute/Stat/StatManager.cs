using System;
using UnityEngine;

namespace LooCast.Attribute.Stat
{
    using LooCast.System;
    using LooCast.System.Management;
    
    public class StatManager : SubModuleManager
    {
        #region Static Properties
        public static StatManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[StatManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = AttributeManager.Instance.transform;
                    return instanceObject.AddComponent<StatManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static StatManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            Namespace attributeNamespace = namespaceManager.GetNamespace("LooCast.Attribute");
            looCastNamespace = new Namespace("Stat", attributeNamespace);
            looCastType = new Type(typeof(StatManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            
            Type statType = new Type(typeof(Stat), looCastNamespace);
            Type statsType = new Type(typeof(Stats), looCastNamespace);
            Type agilityStatType = new Type(typeof(AgilityStat), looCastNamespace);
            Type alertnessStatType = new Type(typeof(AlertnessStat), looCastNamespace);
            Type awarenessStatType = new Type(typeof(AwarenessStat), looCastNamespace);
            Type bodyStatType = new Type(typeof(BodyStat), looCastNamespace);
            Type brawnStatType = new Type(typeof(BrawnStat), looCastNamespace);
            Type cautiousnessStatType = new Type(typeof(CautiousnessStat), looCastNamespace);
            Type chanceStatType = new Type(typeof(ChanceStat), looCastNamespace);
            Type charmStatType = new Type(typeof(CharmStat), looCastNamespace);
            Type egoStatType = new Type(typeof(EgoStat), looCastNamespace);
            Type enduranceStatType = new Type(typeof(EnduranceStat), looCastNamespace);
            Type fateStatType = new Type(typeof(FateStat), looCastNamespace);
            Type fortitudeStatType = new Type(typeof(FortitudeStat), looCastNamespace);
            Type fortuneStatType = new Type(typeof(FortuneStat), looCastNamespace);
            Type intellectStatType = new Type(typeof(IntellectStat), looCastNamespace);
            Type knowledgeStatType = new Type(typeof(KnowledgeStat), looCastNamespace);
            Type mightStatType = new Type(typeof(MightStat), looCastNamespace);
            Type mindStatType = new Type(typeof(MindStat), looCastNamespace);
            Type personalityStatType = new Type(typeof(PersonalityStat), looCastNamespace);
            Type powerStatType = new Type(typeof(PowerStat), looCastNamespace);
            Type presenceStatType = new Type(typeof(PresenceStat), looCastNamespace);
            Type psycheStatType = new Type(typeof(PsycheStat), looCastNamespace);
            Type quicknessStatType = new Type(typeof(QuicknessStat), looCastNamespace);
            Type recoveryStatType = new Type(typeof(RecoveryStat), looCastNamespace);
            Type reflexesStatType = new Type(typeof(ReflexesStat), looCastNamespace);
            Type resilienceStatType = new Type(typeof(ResilienceStat), looCastNamespace);
            Type resistanceStatType = new Type(typeof(ResistanceStat), looCastNamespace);
            Type resolveStatType = new Type(typeof(ResolveStat), looCastNamespace);
            Type sanityStatType = new Type(typeof(SanityStat), looCastNamespace);
            Type senseStatType = new Type(typeof(SenseStat), looCastNamespace);
            Type socialStatType = new Type(typeof(SocialStat), looCastNamespace);
            Type spiritStatType = new Type(typeof(SpiritStat), looCastNamespace);
            Type staminaStatType = new Type(typeof(StaminaStat), looCastNamespace);
            Type vitalityStatType = new Type(typeof(VitalityStat), looCastNamespace);
            Type witsStatType = new Type(typeof(WitsStat), looCastNamespace);

            typeManager.RegisterType(statType);
            typeManager.RegisterType(statsType);
            typeManager.RegisterType(agilityStatType);
            typeManager.RegisterType(alertnessStatType);
            typeManager.RegisterType(awarenessStatType);
            typeManager.RegisterType(bodyStatType);
            typeManager.RegisterType(brawnStatType);
            typeManager.RegisterType(cautiousnessStatType);
            typeManager.RegisterType(chanceStatType);
            typeManager.RegisterType(charmStatType);
            typeManager.RegisterType(egoStatType);
            typeManager.RegisterType(enduranceStatType);
            typeManager.RegisterType(fateStatType);
            typeManager.RegisterType(fortitudeStatType);
            typeManager.RegisterType(fortuneStatType);
            typeManager.RegisterType(intellectStatType);
            typeManager.RegisterType(knowledgeStatType);
            typeManager.RegisterType(mightStatType);
            typeManager.RegisterType(mindStatType);
            typeManager.RegisterType(personalityStatType);
            typeManager.RegisterType(powerStatType);
            typeManager.RegisterType(presenceStatType);
            typeManager.RegisterType(psycheStatType);
            typeManager.RegisterType(quicknessStatType);
            typeManager.RegisterType(recoveryStatType);
            typeManager.RegisterType(reflexesStatType);
            typeManager.RegisterType(resilienceStatType);
            typeManager.RegisterType(resistanceStatType);
            typeManager.RegisterType(resolveStatType);
            typeManager.RegisterType(sanityStatType);
            typeManager.RegisterType(senseStatType);
            typeManager.RegisterType(socialStatType);
            typeManager.RegisterType(spiritStatType);
            typeManager.RegisterType(staminaStatType);
            typeManager.RegisterType(vitalityStatType);
            typeManager.RegisterType(witsStatType);
            #endregion
        }
        #endregion
    }
}