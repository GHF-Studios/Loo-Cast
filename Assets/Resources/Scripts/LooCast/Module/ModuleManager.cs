using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Module
{
    using Mod;
    
    public class ModuleManager
    {
        #region Static Properties
        public static ModuleManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ModuleManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ModuleManager instance;
        #endregion

        #region Properties
        public Dictionary<string, Module> LoadedModules => loadedModules;
        #endregion

        #region Fields
        private Dictionary<string, Module> loadedModules;
        #endregion

        #region Methods
        internal void Initialize()
        {
            LoadAllModules();
            ValidateAllModules();
        }

        internal void OnPreInitialize()
        {
            Debug.Log($"[ModuleManager] Starting Pre-Initialization.");

            foreach (Module loadedModule in loadedModules.Values)
            {
                loadedModule.OnPreInitialize();
            }

            Debug.Log($"[ModuleManager] Finished Pre-Initialization.");
        }

        internal void OnInitialize()
        {
            Debug.Log($"[ModuleManager] Starting Initialization.");

            foreach (Module loadedModule in loadedModules.Values)
            {
                loadedModule.OnInitialize();
            }

            Debug.Log($"[ModuleManager] Finished Initialization.");
        }

        internal void OnPostInitialize()
        {
            Debug.Log($"[ModuleManager] Starting Post-Initialization.");

            foreach (Module loadedModule in loadedModules.Values)
            {
                loadedModule.OnPostInitialize();
            }

            Debug.Log($"[ModuleManager] Finished Post-Initialization.");
        }

        internal void SaveModule(Module module)
        {
            string path = Path.Combine(module.FolderPath, $"{module.Name}.LCMODULE");
            string json = JsonUtility.ToJson(module, true);
            File.WriteAllText(path, json);
        }

        private void LoadModule(FileInfo moduleInfoFile)
        {
            if (moduleInfoFile.Exists)
            {
                string json = File.ReadAllText(moduleInfoFile.FullName);
                Module module = JsonUtility.FromJson<Module>(json);
                loadedModules.Add(module.Name, module);
            }
            else
            {
                throw new FileNotFoundException($"[ModuleManager] Module Info '{moduleInfoFile.Name}' could not be found!");
            }
        }

        private void LoadModules(Mod mod)
        {
            DirectoryInfo modulesFolder = new DirectoryInfo(mod.ModulesFolderPath);
            DirectoryInfo[] moduleFolders = modulesFolder.GetDirectories();
            FileInfo[] moduleInfoFiles = new FileInfo[moduleFolders.Length];
            for (int i = 0; i < moduleFolders.Length; i++)
            {
                moduleInfoFiles[i] = new FileInfo(Path.Combine(moduleFolders[i].FullName, $"{moduleFolders[i].Name}.LCMODULE"));
            }
            foreach (FileInfo moduleInfoFile in moduleInfoFiles)
            {
                LoadModule(moduleInfoFile);
            }
        }

        private void LoadAllModules()
        {
            foreach (Mod mod in ModManager.Instance.LoadedMods.Values)
            {
                LoadModules(mod);
            }
        }

        private void ValidateAllModules()
        {
            foreach (Module loadedModule in loadedModules.Values)
            {
                loadedModule.Validate();
            }
        }
        #endregion
    }
}
