using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Core
{
    using LooCast.Util;

    [Serializable]
    public abstract class Mod
    {
        #region Static Properties
        #endregion

        #region Properties
        public string FolderPath
        {
            get
            {
                return GetModFolderPath(Name);
            }
        }

        public string Name { get; private set; }
        public string DisplayName { get; private set; }
        public string Author { get; private set; }
        public string Version { get; private set; }
        public string GameVersion { get; private set; }
        public string Description { get; private set; }
        public string[] Dependencies { get; private set; }
        public string[] Conflicts { get; private set; }
        public string[] ModuleNames { get; private set; }
        public List<Module> Modules { get; private set; }
        #endregion

        #region Static Methods
        public static string GetModFolderPath(string modName)
        {
            return Path.Combine(ModManager.ModsFolderPath, modName);
        }

        private static Module DeserializeModule(string modulePath)
        {
            if (File.Exists(modulePath))
            {
                string json = File.ReadAllText(modulePath);
                return JsonUtility.FromJson<Module>(json);
            }
            else
            {
                throw new FileNotFoundException($"Module at {modulePath} could not be found!");
            }
        }
        #endregion

        #region Methods
        public virtual void Initialize()
        {
            for (int i = 0; i < Modules.Count; i++)
            {
                Modules[i].Initialize();
            }
        }

        public virtual void OnPreInitialize()
        {
            DeserializeModules();
            
            for (int i = 0; i < Modules.Count; i++)
            {
                ModuleManager.Instance.RegisterModule(Modules[i]);
            }

            for (int i = 0; i < Modules.Count; i++)
            {
                Modules[i].OnPreInitialize();
            }
        }

        public virtual void OnPostInitialize()
        {
            for (int i = 0; i < Modules.Count; i++)
            {
                Modules[i].OnPostInitialize();
            }
        }

        private string GetModulePath(string moduleName)
        {
            return Path.Combine(FolderPath, "Modules", moduleName);
        }

        private void DeserializeModules()
        {
            Modules = new List<Module>();
            foreach (string moduleName in ModuleNames)
            {
                Module module = DeserializeModule(GetModulePath(moduleName));
                Modules.Add(module);
            }
        }
        #endregion
    }
}
