using System;
using UnityEngine;

namespace LooCast.UI.Slider
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UISliderManager : SubModuleManager
    {
        #region Static Properties
        public static UISliderManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UISliderManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UISliderManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UISliderManager instance;
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

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Slider", rootNamespace);
            looCastType = new Type(typeof(UISliderManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type difficultySliderType = new Type(typeof(DifficultySlider), looCastNamespace);
            Type effectVolumeSliderType = new Type(typeof(EffectVolumeSlider), looCastNamespace);
            Type masterVolumeSliderType = new Type(typeof(MasterVolumeSlider), looCastNamespace);
            Type musicVolumeSliderType = new Type(typeof(MusicVolumeSlider), looCastNamespace);
            Type sliderType = new Type(typeof(Slider), looCastNamespace);
            Type uiVolumeSliderType = new Type(typeof(UIVolumeSlider), looCastNamespace);
            Type volumeSliderType = new Type(typeof(VolumeSlider), looCastNamespace);

            typeManager.RegisterType(difficultySliderType);
            typeManager.RegisterType(effectVolumeSliderType);
            typeManager.RegisterType(masterVolumeSliderType);
            typeManager.RegisterType(musicVolumeSliderType);
            typeManager.RegisterType(sliderType);
            typeManager.RegisterType(uiVolumeSliderType);
            typeManager.RegisterType(volumeSliderType);
            #endregion
        }
        #endregion
    }
}