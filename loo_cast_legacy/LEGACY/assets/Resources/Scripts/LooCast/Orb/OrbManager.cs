using System;
using UnityEngine;

namespace LooCast.Orb
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class OrbManager : ModuleManager
    {
        #region Static Properties
        public static OrbManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[OrbManager]");
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Orb", rootNamespace);
            looCastType = new Type(typeof(OrbManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type experienceOrbType = new Type(typeof(ExperienceOrb), looCastNamespace);
            Type magnetOrbType = new Type(typeof(MagnetOrb), looCastNamespace);

            typeManager.RegisterType(experienceOrbType);
            typeManager.RegisterType(magnetOrbType);
            #endregion
        }
        #endregion
    }
}