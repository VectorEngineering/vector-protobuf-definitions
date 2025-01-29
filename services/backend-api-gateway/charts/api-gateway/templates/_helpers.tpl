{{/*
Expand the name of the chart.
*/}}
{{- define "api-gateway.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "api-gateway.fullname" -}}
{{- $values := default dict .Values -}}
{{- $chart := default dict .Chart -}}
{{- $release := default dict .Release -}}
{{- $releaseName := default "release-name" (get $release "Name") -}}
{{- if and $values (hasKey $values "fullnameOverride") }}
{{- $values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default "api-gateway" (default (get $chart "Name") (default "" (get $values "nameOverride"))) }}
{{- if contains $name $releaseName }}
{{- $releaseName | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" $releaseName $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "api-gateway.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "api-gateway.labels" -}}
helm.sh/chart: {{ include "api-gateway.chart" . }}
{{ include "api-gateway.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/instance: {{.Release.Name}}
app.kubernetes.io/managed-by: {{.Release.Service}}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "api-gateway.selectorLabels" -}}
app.kubernetes.io/name: {{ include "api-gateway.fullname" . }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "api-gateway.serviceAccountName" -}}
{{- if .Values.serviceAccount.enabled }}
{{- default (include "api-gateway.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Create the name of the tls secret for secure port
*/}}
{{- define "api-gateway.tlsSecretName" -}}
{{- $fullname := include "api-gateway.fullname" . -}}
{{- default (printf "%s-tls" $fullname) .Values.tls.secretName }}
{{- end }}

{{/*
Common annotations
*/}}
{{- define "api-gateway.annotations" -}}
helm.sh/chart: {{ include "api-gateway.chart" . }}
app.kubernetes.io/version: {{ default "latest" .Chart.AppVersion | quote }}
app.kubernetes.io/managed-by: {{ default "Helm" .Release.Service }}
meta.helm.sh/release-name: {{ .Release.Name | default "release-name" }}
meta.helm.sh/release-namespace: {{ .Release.Namespace | default "default" }}
service.beta.kubernetes.io/aws-load-balancer-type: external
service.beta.kubernetes.io/aws-load-balancer-nlb-target-type: ip
service.beta.kubernetes.io/aws-load-balancer-scheme: internet-facing
{{- if and .Values.serviceMonitor .Values.serviceMonitor.annotations }}
{{- toYaml .Values.serviceMonitor.annotations | nindent 0 }}
{{- end }}
{{- end }}

{{/*
Selector annotations
*/}}
{{- define "api-gateway.selectorAnnotations" -}}
app.kubernetes.io/name: {{ include "api-gateway.name" . }}
app.kubernetes.io/instance: {{ .Release.Name | default "release-name" }}
{{- end }}
