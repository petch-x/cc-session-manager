import { useEffect, useState } from "react";
import {
  Trash2,
  Folder,
  RefreshCw,
  AlertTriangle,
  CheckCircle,
  XCircle,
  Clock,
  HardDrive,
  Layers,
  FileText,
  ChevronLeft,
  Maximize2,
  Minimize2,
} from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { api } from "./services/api";
import type { ProjectDto, SessionDto, StatisticsDto } from "./types";

type View =
  | "dashboard"
  | "projects"
  | "sessions"
  | "delete-by-age"
  | "session-detail";

function App() {
  const [view, setView] = useState<View>("dashboard");
  const [stats, setStats] = useState<StatisticsDto | null>(null);
  const [projects, setProjects] = useState<ProjectDto[]>([]);
  const [selectedProject, setSelectedProject] = useState<ProjectDto | null>(
    null
  );
  const [selectedSessions, setSelectedSessions] = useState<Set<string>>(
    new Set()
  );
  const [selectedSession, setSelectedSession] = useState<SessionDto | null>(
    null
  );
  const [deleteAge, setDeleteAge] = useState<number>(30);
  const [oldSessions, setOldSessions] = useState<SessionDto[]>([]);
  const [selectedOldSessions, setSelectedOldSessions] = useState<Set<string>>(
    new Set()
  );
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingContent, setIsLoadingContent] = useState(false);
  const [sessionFullContent, setSessionFullContent] = useState<string>("");
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);
  const [contentExpanded, setContentExpanded] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setIsLoading(true);
    try {
      const [statsData, projectsData] = await Promise.all([
        api.getStatistics(),
        api.scanProjects(),
      ]);
      setStats(statsData);
      setProjects(projectsData);
    } catch (error) {
      console.error("Failed to load data:", error);
      setMessage({ type: "error", text: "Failed to load data" });
    } finally {
      setIsLoading(false);
    }
  };

  const showMessage = (type: "success" | "error", text: string) => {
    setMessage({ type, text });
    setTimeout(() => setMessage(null), 3000);
  };

  const handleSelectProject = (project: ProjectDto) => {
    setSelectedProject(project);
    setSelectedSessions(new Set());
    setView("sessions");
  };

  const handleToggleSession = (sessionPath: string) => {
    const newSelected = new Set(selectedSessions);
    if (newSelected.has(sessionPath)) {
      newSelected.delete(sessionPath);
    } else {
      newSelected.add(sessionPath);
    }
    setSelectedSessions(newSelected);
  };

  const handleSelectSession = async (session: SessionDto) => {
    setSelectedSession(session);
    setSessionFullContent("");
    setIsLoadingContent(true);
    try {
      const content = await api.getSessionContent(session.path);
      setSessionFullContent(content);
    } catch (error) {
      console.error("Failed to load session content:", error);
      setSessionFullContent("");
    } finally {
      setIsLoadingContent(false);
    }
    setView("session-detail");
  };

  const handleSelectAllSessions = () => {
    if (selectedProject) {
      if (selectedSessions.size === selectedProject.sessions.length) {
        setSelectedSessions(new Set());
      } else {
        setSelectedSessions(
          new Set(selectedProject.sessions.map((s) => s.path))
        );
      }
    }
  };

  const handleDeleteSessions = async () => {
    if (selectedSessions.size === 0) return;
    setIsLoading(true);
    try {
      const count = await api.deleteSessions(Array.from(selectedSessions));
      showMessage("success", `Deleted ${count} sessions`);
      setSelectedSessions(new Set());
      await loadData();
      if (selectedProject) {
        const updatedProject = projects.find(
          (p) => p.path === selectedProject.path
        );
        if (updatedProject) {
          setSelectedProject(updatedProject);
        } else {
          setView("projects");
        }
      }
    } catch (error) {
      showMessage("error", "Failed to delete sessions");
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteProject = async () => {
    if (!selectedProject) return;
    setIsLoading(true);
    try {
      await api.deleteProject(selectedProject.path);
      showMessage("success", `Deleted project: ${selectedProject.name}`);
      setSelectedProject(null);
      await loadData();
    } catch (error) {
      showMessage("error", "Failed to delete project");
    } finally {
      setIsLoading(false);
    }
  };

  const handleLoadOldSessions = async () => {
    setIsLoading(true);
    try {
      const sessions = await api.filterSessionsByAge(deleteAge);
      setOldSessions(sessions);
      setSelectedOldSessions(new Set());
    } catch (error) {
      showMessage("error", "Failed to filter sessions");
    } finally {
      setIsLoading(false);
    }
  };

  const handleToggleOldSession = (sessionPath: string) => {
    const newSelected = new Set(selectedOldSessions);
    if (newSelected.has(sessionPath)) {
      newSelected.delete(sessionPath);
    } else {
      newSelected.add(sessionPath);
    }
    setSelectedOldSessions(newSelected);
  };

  const handleSelectAllOldSessions = () => {
    if (selectedOldSessions.size === oldSessions.length) {
      setSelectedOldSessions(new Set());
    } else {
      setSelectedOldSessions(new Set(oldSessions.map((s) => s.path)));
    }
  };

  const handleDeleteOldSessions = async () => {
    if (selectedOldSessions.size === 0) return;
    setIsLoading(true);
    try {
      const count = await api.deleteSessions(Array.from(selectedOldSessions));
      showMessage("success", `Deleted ${count} sessions`);
      setSelectedOldSessions(new Set());
      await handleLoadOldSessions();
      await loadData();
    } catch (error) {
      showMessage("error", "Failed to delete sessions");
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteAllOldSessions = async () => {
    if (oldSessions.length === 0) return;
    setIsLoading(true);
    try {
      const count = await api.deleteOldSessions(deleteAge);
      showMessage("success", `Deleted ${count} sessions`);
      setOldSessions([]);
      setSelectedOldSessions(new Set());
      await loadData();
    } catch (error) {
      showMessage("error", "Failed to delete sessions");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen p-6 overflow-hidden">
      {message && (
        <div
          className={`fixed top-4 right-4 z-50 flex items-center gap-2 px-4 py-3 rounded-lg glass shadow-lg ${
            message.type === "success"
              ? "text-green-400 border-green-500/30"
              : "text-red-400 border-red-500/30"
          }`}
        >
          {message.type === "success" ? (
            <CheckCircle className="w-4 h-4" />
          ) : (
            <XCircle className="w-4 h-4" />
          )}
          {message.text}
        </div>
      )}

      <div className="w-full mx-auto h-[calc(100vh-48px)]">
        {view === "dashboard" && (
          <>
            <header className="mb-8">
              <div className="glass-light rounded-2xl p-6 mb-6 flex items-center gap-4">
                <img
                  src="/logo.png"
                  alt="Logo"
                  className="w-16 h-16 rounded-xl object-contain"
                />
                <div>
                  <h1 className="text-3xl font-bold text-foreground mb-2 bg-gradient-to-r from-teal-400 to-emerald-400 bg-clip-text text-transparent">
                    Claude Code Session Manager
                  </h1>
                  <p className="text-muted-foreground">
                    Manage your Claude Code sessions across all projects
                  </p>
                </div>
              </div>
            </header>

            <div className="space-y-6">
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <Card glass className="glass-hover">
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium text-muted-foreground">
                      Projects
                    </CardTitle>
                    <Layers className="h-5 w-5 text-teal-500" />
                  </CardHeader>
                  <CardContent>
                    <div className="text-3xl font-bold text-foreground">
                      {stats?.total_projects || 0}
                    </div>
                  </CardContent>
                </Card>
                <Card glass className="glass-hover">
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium text-muted-foreground">
                      Sessions
                    </CardTitle>
                    <Folder className="h-5 w-5 text-teal-500" />
                  </CardHeader>
                  <CardContent>
                    <div className="text-3xl font-bold text-foreground">
                      {stats?.total_sessions || 0}
                    </div>
                  </CardContent>
                </Card>
                <Card glass className="glass-hover">
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-sm font-medium text-muted-foreground">
                      Total Size
                    </CardTitle>
                    <HardDrive className="h-5 w-5 text-teal-500" />
                  </CardHeader>
                  <CardContent>
                    <div className="text-3xl font-bold text-foreground">
                      {stats?.total_size || "0 B"}
                    </div>
                  </CardContent>
                </Card>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <Card
                  glass
                  className="cursor-pointer glass-hover"
                  onClick={() => setView("projects")}
                >
                  <CardHeader>
                    <CardTitle className="flex items-center gap-2 text-foreground">
                      <Folder className="h-5 w-5 text-teal-500" />
                      Manage Projects
                    </CardTitle>
                    <CardDescription>
                      View and manage sessions by project
                    </CardDescription>
                  </CardHeader>
                  <CardContent>
                    <p className="text-sm text-muted-foreground">
                      Select a project to view its sessions and delete selected
                      ones.
                    </p>
                  </CardContent>
                </Card>

                <Card
                  glass
                  className="cursor-pointer glass-hover"
                  onClick={() => {
                    handleLoadOldSessions();
                    setView("delete-by-age");
                  }}
                >
                  <CardHeader>
                    <CardTitle className="flex items-center gap-2 text-foreground">
                      <Clock className="h-5 w-5 text-teal-500" />
                      Delete by Age
                    </CardTitle>
                    <CardDescription>
                      Remove sessions older than a certain number of days
                    </CardDescription>
                  </CardHeader>
                  <CardContent>
                    <p className="text-sm text-muted-foreground">
                      Find and delete sessions based on their age.
                    </p>
                  </CardContent>
                </Card>
              </div>

              <div className="flex justify-center">
                <Button variant="glass" onClick={loadData} disabled={isLoading}>
                  <RefreshCw
                    className={`w-4 h-4 mr-2 ${
                      isLoading ? "animate-spin" : ""
                    }`}
                  />
                  Refresh
                </Button>
              </div>
            </div>
          </>
        )}

        {view === "projects" && (
          <div className="flex flex-col h-[calc(100vh-50px)]">
            <div className="flex items-center gap-4 mb-4 shrink-0">
              <Button variant="glass" onClick={() => setView("dashboard")}>
                ← Back to Dashboard
              </Button>
              <h2 className="text-xl font-semibold text-foreground">
                Projects ({projects.length})
              </h2>
            </div>

            <ScrollArea className="flex-1 rounded-xl border-0">
              <div className="p-2 space-y-2">
                {projects.map((project) => (
                  <Card
                    key={project.path}
                    glass
                    className="cursor-pointer glass-hover"
                    onClick={() => handleSelectProject(project)}
                  >
                    <CardContent className="flex items-center justify-between p-4">
                      <div>
                        <h3 className="font-semibold text-foreground">
                          {project.name}
                        </h3>
                        <p className="text-sm text-muted-foreground">
                          {project.session_count} sessions •{" "}
                          {project.total_size}
                        </p>
                      </div>
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={(e) => {
                          e.stopPropagation();
                          setSelectedProject(project);
                        }}
                      >
                        <Trash2 className="w-4 h-4 mr-1" />
                        Delete Project
                      </Button>
                    </CardContent>
                  </Card>
                ))}
                {projects.length === 0 && (
                  <div className="glass rounded-lg p-8 text-center">
                    <p className="text-muted-foreground py-8">
                      No projects found
                    </p>
                  </div>
                )}
              </div>
            </ScrollArea>
          </div>
        )}

        {view === "sessions" && selectedProject && (
          <div className="flex flex-col h-[calc(100vh-50px)]">
            <div className="flex items-center gap-4 mb-4 shrink-0">
              <Button
                variant="glass"
                onClick={() => {
                  setView("projects");
                  setSelectedProject(null);
                }}
              >
                ← Back to Projects
              </Button>
              <h2 className="text-xl font-semibold text-foreground">
                Sessions in "{selectedProject.name}" ({selectedSessions.size}/
                {selectedProject.sessions.length})
              </h2>
            </div>

            <div className="flex items-center gap-2 mb-4 shrink-0">
              <Button
                variant="glass"
                size="sm"
                onClick={handleSelectAllSessions}
              >
                {selectedSessions.size === selectedProject.sessions.length
                  ? "Deselect All"
                  : "Select All"}
              </Button>
              <Button
                variant="destructive"
                size="sm"
                onClick={handleDeleteSessions}
                disabled={selectedSessions.size === 0}
              >
                <Trash2 className="w-4 h-4 mr-1" />
                Delete Selected ({selectedSessions.size})
              </Button>
            </div>

            <ScrollArea className="flex-1 rounded-xl border-0">
              <div className="p-2 space-y-2">
                {selectedProject.sessions.map((session) => (
                  <Card
                    key={session.path}
                    glass
                    className="cursor-pointer glass-hover"
                    onClick={() => handleSelectSession(session)}
                  >
                    <CardContent className="flex items-start gap-3 p-3">
                      <div onClick={(e) => e.stopPropagation()}>
                        <Checkbox
                          checked={selectedSessions.has(session.path)}
                          onCheckedChange={() =>
                            handleToggleSession(session.path)
                          }
                        />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center gap-2 flex-wrap">
                          <h4 className="font-medium text-foreground truncate">
                            {session.name}
                          </h4>
                          <Badge variant="secondary">{session.size}</Badge>
                          <Badge variant="outline">
                            {session.age_days} days ago
                          </Badge>
                        </div>
                        {session.content_preview && (
                          <p className="text-sm text-muted-foreground mt-1 line-clamp-10">
                            {session.content_preview}
                          </p>
                        )}
                      </div>
                    </CardContent>
                  </Card>
                ))}
              </div>
            </ScrollArea>
          </div>
        )}

        {view === "delete-by-age" && (
          <div className="flex flex-col h-[calc(100vh-50px)]">
            <div className="flex items-center gap-4 mb-4 shrink-0">
              <Button variant="glass" onClick={() => setView("dashboard")}>
                ← Back to Dashboard
              </Button>
              <h2 className="text-xl font-semibold text-foreground">
                Delete by Age
              </h2>
            </div>

            <Card glass className="shrink-0">
              <CardContent className="pt-4 pb-4">
                <div className="flex items-center gap-4 flex-wrap">
                  <label className="text-sm font-medium text-muted-foreground">
                    Delete sessions older than
                  </label>
                  <input
                    type="number"
                    value={deleteAge}
                    onChange={(e) =>
                      setDeleteAge(parseInt(e.target.value) || 0)
                    }
                    className="glass-input w-20 text-center"
                    min="1"
                  />
                  <label className="text-sm font-medium text-muted-foreground">
                    days
                  </label>
                  <Button
                    variant="glass"
                    onClick={handleLoadOldSessions}
                    disabled={isLoading}
                  >
                    <RefreshCw
                      className={`w-4 h-4 mr-2 ${
                        isLoading ? "animate-spin" : ""
                      }`}
                    />
                    Find Sessions
                  </Button>
                </div>
              </CardContent>
            </Card>

            {oldSessions.length > 0 && (
              <div className="flex flex-col flex-1 min-h-0 mt-4">
                <div className="flex items-center gap-2 flex-wrap mb-4 shrink-0">
                  <Button
                    variant="glass"
                    size="sm"
                    onClick={handleSelectAllOldSessions}
                  >
                    {selectedOldSessions.size === oldSessions.length
                      ? "Deselect All"
                      : "Select All"}
                  </Button>
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={handleDeleteOldSessions}
                    disabled={selectedOldSessions.size === 0}
                  >
                    <Trash2 className="w-4 h-4 mr-1" />
                    Delete Selected ({selectedOldSessions.size})
                  </Button>
                  <Button
                    variant="destructive"
                    size="sm"
                    onClick={handleDeleteAllOldSessions}
                  >
                    <AlertTriangle className="w-4 h-4 mr-1" />
                    Delete All ({oldSessions.length})
                  </Button>
                </div>

                <ScrollArea className="flex-1 rounded-xl border-0">
                  <div className="p-2 space-y-2">
                    {oldSessions.map((session) => (
                      <Card key={session.path} glass className="glass-hover">
                        <CardContent className="flex items-start gap-3 p-3">
                          <Checkbox
                            checked={selectedOldSessions.has(session.path)}
                            onCheckedChange={() =>
                              handleToggleOldSession(session.path)
                            }
                          />
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2">
                              <h4 className="font-medium text-foreground truncate">
                                {session.name}
                              </h4>
                              <Badge variant="secondary">{session.size}</Badge>
                              <Badge variant="outline">
                                {session.age_days} days ago
                              </Badge>
                            </div>
                            {session.content_preview && (
                              <p className="text-sm text-muted-foreground mt-1 line-clamp-10">
                                {session.content_preview}
                              </p>
                            )}
                          </div>
                        </CardContent>
                      </Card>
                    ))}
                  </div>
                </ScrollArea>
              </div>
            )}
          </div>
        )}

        {view === "session-detail" && selectedSession && (
          <div className="flex flex-col h-[calc(100vh-50px)]">
            <div className="flex items-center gap-4 mb-4 shrink-0">
              <Button
                variant="glass"
                onClick={() => {
                  setView("sessions");
                  setSelectedSession(null);
                }}
              >
                <ChevronLeft className="w-4 h-4 mr-2" />
                Back to Sessions
              </Button>
              <h2 className="text-xl font-semibold text-foreground flex items-center gap-2">
                <FileText className="h-5 w-5 text-teal-500" />
                Session Details
              </h2>
            </div>

            {!contentExpanded && (
              <Card glass className="shrink-0">
                <CardContent className="pt-4 pb-4">
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                      <span className="text-muted-foreground">Name:</span>
                      <p className="font-medium text-foreground">
                        {selectedSession.name}
                      </p>
                    </div>
                    <div>
                      <span className="text-muted-foreground">Size:</span>
                      <p className="font-medium text-foreground">
                        {selectedSession.size}
                      </p>
                    </div>
                    <div>
                      <span className="text-muted-foreground">Age:</span>
                      <p className="font-medium text-foreground">
                        {selectedSession.age_days} days ago
                      </p>
                    </div>
                    <div className="col-span-2">
                      <span className="text-muted-foreground">Path:</span>
                      <p className="font-medium text-foreground truncate">
                        {selectedSession.path}
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            )}

            <div className="flex-1 flex flex-col min-h-0 mt-4">
              <div className="flex items-center justify-between mb-2 shrink-0">
                <span className="text-sm text-muted-foreground">Content:</span>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setContentExpanded(!contentExpanded)}
                  className="text-muted-foreground hover:text-foreground"
                >
                  {contentExpanded ? (
                    <Minimize2 className="w-4 h-4" />
                  ) : (
                    <Maximize2 className="w-4 h-4" />
                  )}
                </Button>
              </div>
              {isLoadingContent ? (
                <div className="flex-1 flex items-center justify-center glass rounded-lg">
                  <RefreshCw className="h-6 w-6 animate-spin text-teal-500" />
                  <span className="ml-2 text-muted-foreground">
                    Loading content...
                  </span>
                </div>
              ) : (
                <ScrollArea
                  className={
                    contentExpanded
                      ? "h-[calc(100vh-80px)] rounded-xl border-0"
                      : "flex-1 rounded-xl border-0"
                  }
                >
                  <div className="glass rounded-lg p-4 min-h-full">
                    <pre className="text-sm whitespace-pre-wrap font-mono break-all w-full text-foreground">
                      {sessionFullContent ||
                        selectedSession.content_preview ||
                        "No content available"}
                    </pre>
                  </div>
                </ScrollArea>
              )}
            </div>
          </div>
        )}

        <Dialog
          open={!!selectedProject && view === "projects"}
          onOpenChange={() => setSelectedProject(null)}
        >
          <DialogContent className="glass">
            <DialogHeader>
              <DialogTitle className="flex items-center gap-2 text-red-400">
                <AlertTriangle className="h-5 w-5" />
                Delete Project
              </DialogTitle>
              <DialogDescription>
                Are you sure you want to delete the project "
                {selectedProject?.name}"? This will permanently delete all
                sessions in this project and cannot be undone.
              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <Button variant="glass" onClick={() => setSelectedProject(null)}>
                Cancel
              </Button>
              <Button
                variant="destructive"
                onClick={handleDeleteProject}
                disabled={isLoading}
              >
                Delete Project
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
}

export default App;
