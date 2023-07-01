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
    }
    #endregion

    #region Overrides
    protected override void Expand()
    {
        base.Expand();
    }
    
    protected override void Collapse()
    {
        base.Collapse();
    }
    #endregion
}
