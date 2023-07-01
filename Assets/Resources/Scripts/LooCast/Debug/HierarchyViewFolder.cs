using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyViewFolder : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;

    private IFolder hierarchyFolder;
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    #endregion

    #region Methods
    public void Initialize(IFolder hierarchyFolder)
    {
        base.Initialize(hierarchyFolder.FolderName);
        
        this.hierarchyFolder = hierarchyFolder;
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();

        InstantiateChildren();
    }
    #endregion

    #region Overrides
    protected override void InstantiateChildren()
    {
        foreach (IFolder folder in ((IParent<IFolder>)hierarchyFolder).Children)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, elementContainer.transform).GetComponent<HierarchyViewFolder>();
            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
        }

        foreach (IFile file in ((IParent<IFile>)hierarchyFolder).Children)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, elementContainer.transform).GetComponent<HierarchyViewFile>();
            hierarchyViewFile.gameObject.name = file.FileIdentifier;
            hierarchyViewFileChildren.Add(file.FileIdentifier, hierarchyViewFile);
        }

        if (hierarchyViewFolderChildren.Count == 0 && hierarchyViewFileChildren.Count == 0)
        {
            hasAnyChildren = false;
        }
        else
        {
            hasAnyChildren = true;
        }

        foreach (IFolder folder in ((IParent<IFolder>)hierarchyFolder).Children)
        {
            hierarchyViewFolderChildren.TryGetValue(folder.FolderName, out HierarchyViewFolder hierarchyViewFolder);
            hierarchyViewFolder.Initialize(folder);
        }

        foreach (IFile file in ((IParent<IFile>)hierarchyFolder).Children)
        {
            hierarchyViewFileChildren.TryGetValue(file.FileIdentifier, out HierarchyViewFile hierarchyViewFile);
            hierarchyViewFile.Initialize(file);
        }
    }
    #endregion
}
