using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Module
{
    using Core;
    using Mod;
    
    [Serializable]
    public class Module
    {
        #region Properties
        public string ID
        {
            get
            {
                return $"{ModName}.{Name}";
            }
        }
        public string FolderPath
        {
            get
            {
                return Path.Combine(ModManager.Instance.LoadedMods[ModName].ModulesFolderPath, Name);
            }
        }
        public string ResourcesFolderPath
        {
            get
            {
                return Path.Combine(MainManager.ModsFolderPath, ModName, "Modules", Name, "Resources");
            }
        }

        public string ModName => modName;
        public string Name => name;
        public string DisplayName => displayName;
        public string Description => description;
        public string[] Dependencies => dependencies;
        public string[] Conflicts => conflicts;
        #endregion

        #region Fields
        [SerializeField] private string modName;
        [SerializeField] private string name;
        [SerializeField] private string displayName;
        [SerializeField] private string description;
        [SerializeField] private string[] dependencies;
        [SerializeField] private string[] conflicts;
        #endregion

        #region Constructors
        public Module(string modName, string name, string displayName, string description, string[] dependencies, string[] conflicts)
        {
            this.modName = modName;
            this.name = name;
            this.displayName = displayName;
            this.description = description;
            this.dependencies = dependencies;
            this.conflicts = conflicts;

            Validate();
        }
        #endregion

        #region Methods
        public virtual void OnInitialize()
        {

        }

        public virtual void OnPreInitialize()
        {

        }

        public virtual void OnPostInitialize()
        {

        }

        internal void Validate()
        {
            #region Mod Info Validation
            if (Name == null || Name == "")
            {
                throw new Exception($"[ModuleManager] Module has no name!");
            }
            if (ModName == null || ModName == "")
            {
                throw new Exception($"[ModuleManager] Module '{this}' has no mod name!");
            }
            if (DisplayName == null || DisplayName == "")
            {
                throw new Exception($"[ModuleManager] Module '{this}' has no display name!");
            }
            if (Description == null || Description == "")
            {
                throw new Exception($"[ModuleManager] Module '{this}' has no description!");
            }
            if (Dependencies == null)
            {
                dependencies = new string[0];
            }
            if (Conflicts == null)
            {
                conflicts = new string[0];
            }
            #endregion

            #region File Structure Validation
            DirectoryInfo resourcesFolder = new DirectoryInfo(ResourcesFolderPath);
            if (!resourcesFolder.Exists)
            {
                resourcesFolder.Create();
            }
            #endregion

            #region Conflict Validation
            foreach (string conflict in Conflicts)
            {
                string[] conflictNamespaces = conflict.Split('.');
                if (ModuleManager.Instance.LoadedModules.ContainsKey(conflictNamespaces[0]))
                {
                    throw new Exception($"[ModuleManager] Module '{this}' conflicts with '{conflict}'!");
                }
            }
            #endregion

            #region Dependency Validation
            foreach (string dependency in Dependencies)
            {
                string[] dependencyNamespaces = dependency.Split('.');
                if (!ModuleManager.Instance.LoadedModules.ContainsKey(dependencyNamespaces[0]))
                {
                    throw new Exception($"[ModuleManager] Module '{this}' is missing dependency '{dependency}'!");
                }
            }
            #endregion

            #region Cyclic Dependency Validation
            HashSet<string> visited = new HashSet<string>();
            Stack<string> stack = new Stack<string>();
            visited.Add(Name);
            stack.Push(Name);

            while (stack.Count > 0)
            {
                string currentMod = stack.Pop();
                foreach (string dependency in Dependencies)
                {
                    if (!visited.Contains(dependency))
                    {
                        visited.Add(dependency);
                        stack.Push(dependency);
                    }
                    else
                    {
                        throw new Exception($"[ModuleManager] Module {this} has a circular dependency on {dependency}!");
                    }
                }
            }
            #endregion
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return ID;
        }
        #endregion
    }
}
