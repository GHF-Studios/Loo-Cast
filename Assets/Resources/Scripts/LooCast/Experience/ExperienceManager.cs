using System;
using UnityEngine;

namespace LooCast.Experience
{
    using LooCast.System;
    using LooCast.System.Management;
    
    public class ExperienceManager : ModuleManager
    {
        #region Static Properties
        public static ExperienceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[ExperienceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<ExperienceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ExperienceManager instance;
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
            looCastNamespace = new Namespace("Experience", rootNamespace);
            looCastType = new Type(typeof(ExperienceManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            
            Type iExperienceType = new Type(typeof(IExperience), looCastNamespace);
            Type playerExperienceType = new Type(typeof(PlayerExperience), looCastNamespace);

            typeManager.RegisterType(iExperienceType);
            typeManager.RegisterType(playerExperienceType);
            #endregion
        }
        #endregion
    }
}