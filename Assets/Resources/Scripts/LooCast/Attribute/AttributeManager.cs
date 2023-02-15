using System;
using UnityEngine;

namespace LooCast.Attribute
{
    using Stat;
    
    public class AttributeManager : ModuleManager
    {
        #region Static Properties
        public static AttributeManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[AttributeManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<AttributeManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static AttributeManager instance;
        #endregion

        #region Properties
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
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Attribute", rootNamespace);
            looCastType = new Type(typeof(AttributeManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type attributeType = new Type(typeof(Attribute), looCastNamespace);
            Type attributesType = new Type(typeof(Attributes), looCastNamespace);
            Type charismaAttributeType = new Type(typeof(CharismaAttribute), looCastNamespace);
            Type constitutionAttributeType = new Type(typeof(ConstitutionAttribute), looCastNamespace);
            Type defenseAttributeType = new Type(typeof(DefenseAttribute), looCastNamespace);
            Type dexterityAttributeType = new Type(typeof(DexterityAttribute), looCastNamespace);
            Type intelligenceAttributeType = new Type(typeof(IntelligenceAttribute), looCastNamespace);
            Type luckAttributeType = new Type(typeof(LuckAttribute), looCastNamespace);
            Type perceptionAttributeType = new Type(typeof(PerceptionAttribute), looCastNamespace);
            Type strengthAttributeType = new Type(typeof(StrengthAttribute), looCastNamespace);
            Type willpowerAttributeType = new Type(typeof(WillpowerAttribute), looCastNamespace);
            Type wisdomAttributeType = new Type(typeof(WisdomAttribute), looCastNamespace);

            typeManager.RegisterType(attributeType);
            typeManager.RegisterType(attributesType);
            typeManager.RegisterType(charismaAttributeType);
            typeManager.RegisterType(constitutionAttributeType);
            typeManager.RegisterType(defenseAttributeType);
            typeManager.RegisterType(dexterityAttributeType);
            typeManager.RegisterType(intelligenceAttributeType);
            typeManager.RegisterType(luckAttributeType);
            typeManager.RegisterType(perceptionAttributeType);
            typeManager.RegisterType(strengthAttributeType);
            typeManager.RegisterType(willpowerAttributeType);
            typeManager.RegisterType(wisdomAttributeType);
            #endregion
        }

        protected override SubModuleManager[] GetSubModuleManagers()
        {
            return new SubModuleManager[]
            {
                StatManager.Instance
            };
        }
        #endregion
    }
}