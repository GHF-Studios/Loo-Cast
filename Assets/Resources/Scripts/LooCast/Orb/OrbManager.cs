using System;
using UnityEngine;

namespace LooCast.Orb
{
    public class OrbManager : ModuleManager
    {
        #region Static Properties
        public static OrbManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[OrbManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<OrbManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static OrbManager instance;
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
            looCastNamespace = new Namespace("Orb", rootNamespace);
            looCastType = new Type(typeof(OrbManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type experienceOrbType = new Type(typeof(ExperienceOrb), looCastNamespace);
            Type magnetOrbType = new Type(typeof(MagnetOrb), looCastNamespace);

            typeManager.RegisterType(experienceOrbType);
            typeManager.RegisterType(magnetOrbType);
            #endregion
        }
        #endregion
    }
}